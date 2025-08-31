//backend/src/store.rs
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{Mutex, RwLock},
};
use thiserror::Error;

// Metadata record type i64 int64_t
#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct ReviewMeta {
    pub review_title: String,
    pub review_body:  String,
    pub product_id:   String,
    pub review_rating:i32,
}

// ---- Errors ----
#[derive(Debug, Error)]
pub enum StoreError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid id {0}")]
    InvalidId(u64),
    #[error("index error: {0}")]
    Index(String),
}

// ---- Embedder + Index traits
pub trait VectorIndex: Send + Sync {
    // Append a vector; returns assigned id (0-based).
    fn append(&mut self, vector: &[f32]) -> Result<u64, StoreError>;
    // Search; returns (id, score) pairs sorted by score desc.
    fn search(&self, vector: &[f32], top_k: usize) -> Result<Vec<(u64, f32)>, StoreError>;
    // Optional: number of vectors currently stored.
    fn len(&self) -> Result<u64, StoreError>;
}

// Holds file handles and offsets
pub struct MetadataStore {
    jsonl_path: PathBuf,
    offsets_path: PathBuf,
    // Append file handles guarded by a single mutex to keep ordering atomic.
    jsonl_file: Mutex<File>,
    offsets_file: Mutex<File>,
    // In-memory offsets for fast id -> byte position.
    offsets: RwLock<Vec<u64>>,
}

impl MetadataStore {
    /// Open (or create) `reviews.jsonl` + `reviews.offsets` under 
    pub fn open<P: AsRef<Path>>(dir: P) -> Result<Self, StoreError> {
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir)?;

        let jsonl_path = dir.join("reviews.jsonl");
        let offsets_path = dir.join("reviews.offsets");
        // Open index file /jsonl_path
        let jsonl_file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&jsonl_path)?;
        // Open metadata file
         let mut jsonl_file = OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open(&jsonl_path)?;
        let mut offsets_file = OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open(&offsets_path)?;
        // Load offsets into memory (or rebuild if missing)
        let mut offsets = Vec::new();
        // If offsets file is empty but jsonl has data, rebuild offsets by scanning once.
        let meta_len = offsets_file.metadata()?.len();
        if meta_len == 0 {
            // No offsets on disk yet → scan jsonl
            let mut reader = BufReader::new(File::open(&jsonl_path)?);
            let mut pos: u64 = 0;
            loop {
                offset_record(&mut reader, &mut offsets, &mut pos)?;
                if pos == reader.get_ref().metadata()?.len() {
                    break;
                }
             }
            // Persist rebuilt offsets
            for &off in &offsets {
                offsets_file.write_all(&off.to_le_bytes())?;
            }
            offsets_file.flush()?
        } else {
            // Load existing offsets
            offsets_file.seek(SeekFrom::Start(0))?;
            let mut buf = Vec::new();
            offsets_file.read_to_end(&mut buf)?;
            for chunk in buf.chunks_exact(8) {
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(chunk);
                offsets.push(u64::from_le_bytes(bytes));
            }
            // Seek append handles to end again
            offsets_file.seek(SeekFrom::End(0))?;
        }

        // Ensure jsonl append handle at end
        jsonl_file.seek(SeekFrom::End(0))?;

        Ok(MetadataStore {
            jsonl_path,
            offsets_path,
            jsonl_file: Mutex::new(jsonl_file),
            offsets_file: Mutex::new(offsets_file),
            offsets: RwLock::new(offsets),
        })

    }
   /// Number of records stored
    pub fn len(&self) -> u64 {
        self.offsets.read().unwrap().len() as u64
    }

    /// Append one review: write vector bytes then JSON line
    // Append one review; returns assigned id (line number).
    pub fn append(&self, meta: &ReviewMeta) -> Result<u64, StoreError> {
        // Serialize JSON + newline
        let mut jf = self.jsonl_file.lock().unwrap();
        let mut of = self.offsets_file.lock().unwrap();

        let json_bytes = serde_json::to_vec(meta)?;
        let offset = jf.metadata()?.len();

        jf.write_all(&json_bytes)?;
        jf.write_all(b"\n")?;
        jf.flush()?;

        of.write_all(&offset.to_le_bytes())?;
        of.flush()?;

        let mut offs = self.offsets.write().unwrap();
        offs.push(offset);
        Ok((offs.len() - 1) as u64)

    }

    // Fetch by id using offsets to seek directly
    pub fn get(&self, id: u64) -> Result<ReviewMeta, StoreError> {
        let offs = self.offsets.read().unwrap();
        let idx = id as usize;
        if idx >= offs.len() {
            return Err(StoreError::InvalidId(id));
        }

        let start = offs[idx];
        let end = if idx + 1 < offs.len() {
            offs[idx + 1]
        } else {
            std::fs::metadata(&self.jsonl_path)?.len()
        };

        let mut f = File::open(&self.jsonl_path)?;
        f.seek(SeekFrom::Start(start))?;
        let mut buf = vec![0u8; (end - start) as usize];
        f.read_exact(&mut buf)?;
        // Strip trailing newline just in case
        if buf.last() == Some(&b'\n') {
            buf.pop();
        }

        let meta: ReviewMeta = serde_json::from_slice(&buf)?;
        Ok(meta)
    }
}
/// Helper for rebuilding offsets scanning JSONL
fn offset_record<R: BufRead>(
    reader: &mut R,
    offsets: &mut Vec<u64>,
    pos: &mut u64,
) -> Result<(), std::io::Error> {
    let before = *pos;
    let mut line = String::new();
    let bytes = reader.read_line(&mut line)?;
    if bytes == 0 {
        return Ok(());
    }
    offsets.push(before);
    *pos += bytes as u64;
    Ok(())
}

/// Alias for your handlers & AppState
pub type Store = MetadataStore;
