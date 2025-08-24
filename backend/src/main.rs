//backend/src/main.rs
mod store;  //src/store.rs
mod spfresh_ffi;
mod handler;

use axum::{
    extract::State,
    routing::{get, post},
    Router,
  //  Server,           // ← Hyper server re-exported by Axum
};   
use std::{
  net::SocketAddr,
  sync::{Arc, Mutex},
};

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
  
    // Initialize shared components
   // open (or create) the on-disk index
    let idx = Index::open("data/reviews.index")
        .expect("failed to open or create vector index");

    // open (or create) the metadata files
    let store = Store::open("data")
        .expect("failed to open or create metadata store");

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

   // Start server
  // Bind to a SocketAddr (not &str)
   // let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
   // let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
      //println!("Listening on {}", addr);
//     axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
 let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
  println!("server listen on port : {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}
// }


