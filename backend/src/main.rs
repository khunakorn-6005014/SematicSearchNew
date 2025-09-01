//backend/src/main.rs
mod store;  //src/store.rs
mod spfresh_ffi;
mod handler;

use axum::{
    routing::{get, post},
    Router,
  //  Server,           // ← Hyper server re-exported by Axum int
};   
//use std::sync::{Arc, Mutex};
use std::{path::Path, sync::{Arc, Mutex}};
//use tokio::net::TcpListener;//add
use spfresh_ffi::Index;
use store::Store;
//use crate::embedder::Embedder;
use handler::reviews::{insert_review, insert_bulk_reviews, search_reviews};

/// Centralized state 
#[derive(Clone)]
pub struct AppState {
    /// FFI-backed vector index
    pub index: Arc<Mutex<Index>>,
    /// File-backed metadata store
    pub meta: Arc<Store>,
    /// Single global lock to keep vector+meta appends atomic
    pub commit_lock: Arc<Mutex<()>>,
}

#[tokio::main]
async fn main(){
  //append_raw ,search_raw 
    // Initialize shared components
   // open (or create) the on-disk index
//     let idx = Index::open("data/reviews.index") 
//         .expect("failed to open or create vector index");
//    eprintln!("Opening index at path: {:?}", idx);
       let idx_path = Path::new("data").join("reviews.index");//backend/data/reviews.index,reviews.jsonl
    println!("→ Opening vector index at {:?}", idx_path);
        if let Some(parent) = idx_path.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|e| panic!("failed to create {:?}: {}", parent, e));
    }
    let idx_path_str = idx_path.to_str().unwrap();
    let idx = match Index::open(idx_path_str) {
     Ok(ix) => {
        println!("✔ Loaded existing index");
        ix
    }
    Err(e) => {
        panic!(
                "failed to open vector index {:?}: {:?}",
                idx_path_str, e
            );
    }
};
    println!("✔ Vector index ready");
    // open (or create) the metadata files
   let meta_dir = Path::new("data"); // Make sure "data/" exists
    println!("→ Opening metadata store at {:?}", meta_dir);
    std::fs::create_dir_all(meta_dir)
        .unwrap_or_else(|e| panic!("failed to create {:?}: {}", meta_dir, e));
    let store = Store::open(meta_dir)//Try to open/create and handle errors explicitly
        .expect("failed to open or create metadata store");
    println!("✔ Metadata store ready");
     // Build shared state
    let state = AppState {
        index: Arc::new(Mutex::new(idx)),
        meta: Arc::new(store),
        commit_lock: Arc::new(Mutex::new(())),
    };
    // Build router
   let app = Router::new()
        .route("/reviews",      post(insert_review))
        .route("/reviews/bulk", post(insert_bulk_reviews))
        .route("/search",       post(search_reviews))
        .route("/health",       get(|| async { "OK" }))
        .with_state(state);

 let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
  println!("server listen on port : {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}
// }


