// backend/src/spfresh_ffi.rs
// use std::ffi::c_void;
use std::{ffi::CString,ptr,};
use libc::{c_char, c_float, c_int, int64_t};
use crate::store::StoreError;
#[repr(C)]
pub struct SPFHandleOpaque;
pub type SPFHandle = *mut SPFHandleOpaque;


extern "C" {// C ABI entrypoints from your spfresh_c_api.h
    pub fn spf_init_index(path: *const c_char, out_handle: *mut SPFHandle) -> c_int;
    pub fn spf_add_vector(handle: SPFHandle, vec: *const c_float, dim: c_int, id: int64_t) -> c_int;
    pub fn spf_search(
        handle: SPFHandle,
        query: *const c_float,
        dim: c_int,
        k: c_int,
        out_ids: *mut int64_t,
        out_scores: *mut c_float,
    ) -> c_int;
    pub fn spf_close(handle: SPFHandle) -> c_int;
}

/// Pure-Rust error type for vector indexing.
#[derive(Debug)]
pub enum IndexError {
    CError(c_int),
    NullHandle,
}

/// Safe wrapper around the native handle
pub struct Index {
    handle: SPFHandle,
}

impl Index {
    /// Open (or create) the on-disk vector index
    pub fn open(path: &str) -> Result<Self, IndexError> {
        let c_path = CString::new(path).unwrap();
        let mut handle: SPFHandle = ptr::null_mut();
        let rc = unsafe { spf_init_index(c_path.as_ptr(), &mut handle) };
        if rc != 0 {
            return Err(IndexError::CError(rc));
        }
        if handle.is_null() {
            return Err(IndexError::NullHandle);
        }
        Ok(Index { handle })
    }

    /// Close the native handle when dropped
    fn close(&mut self) {
        if !self.handle.is_null() {
            unsafe { spf_close(self.handle) };
            self.handle = ptr::null_mut();
        }
    }

    /// Append one vector under user-provided `id`
    pub fn append_raw(&mut self, vec: &[f32], id: i64) -> Result<(), IndexError> {
        let rc = unsafe {
            spf_add_vector(
                self.handle,
                vec.as_ptr(),
                vec.len() as c_int,
                id as int64_t,
            )
        };
        if rc != 0 {
            Err(IndexError::CError(rc))
        } else {
            Ok(())
        }
    }

    /// Search top-k, returning (id, score) list
    pub fn search_raw(&self, query: &[f32], k: usize) -> Result<Vec<(usize, f32)>, IndexError> {
        let mut ids = vec![0i64; k];
        let mut scores = vec![0f32; k];
        let rc = unsafe {
            spf_search(
                self.handle,
                query.as_ptr(),
                query.len() as c_int,
                k as c_int,
                ids.as_mut_ptr(),
                scores.as_mut_ptr(),
            )
        };
        if rc != 0 {
            return Err(IndexError::CError(rc));
        }
        Ok(ids
            .into_iter()
            .zip(scores.into_iter())
            .map(|(i, s)| (i as usize, s))
            .collect())
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        self.close();
    }
}

/// Bring your `VectorIndex` trait into scope (from store.rs)
pub use crate::store::VectorIndex;
// SAFETY: We only ever call into the native handle under a Rust Mutex,
// so it is safe to send/share it across threads.
unsafe impl Send for Index {}
unsafe impl Sync for Index {}

/// Implement the trait so you can swap in the FFI index anywhere
impl VectorIndex for Index {
    fn append(&mut self, vector: &[f32]) -> Result<u64, crate::store::StoreError> {
        // Here we use the next line number as ID
        // Alternatively you could track your own counter
        let id = self.search_raw(vector, 1)
            .map_err(|e| StoreError::Index(format!("{:?}", e)))?
            .len() as u64;
        self.append_raw(vector, id as i64)
            .map_err(|e| StoreError::Index(format!("{:?}", e)))?;
        Ok(id)
    }

    fn search(&self, vector: &[f32], top_k: usize) -> Result<Vec<(u64, f32)>, crate::store::StoreError> {
        let hits = self.search_raw(vector, top_k)
            .map_err(|e| StoreError::Index(format!("{:?}", e)))?;
        Ok(hits.into_iter().map(|(i, s)| (i as u64, s)).collect())
    }

    fn len(&self) -> Result<u64, crate::store::StoreError> {
        // You could expose an FFI call for len(), or just error
        Err(StoreError::Index("len() not implemented".into()))
    }
}

