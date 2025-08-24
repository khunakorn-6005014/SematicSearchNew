// src/handlers/reviews.rs

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{AppState, store::ReviewMeta};
use crate::store::Store;
use crate::spfresh_ffi::Index;
use crate::spfresh_ffi::VectorIndex;   

/// Payload for single review
/// Request payload for a single review insert
#[derive(Debug, Deserialize)]
pub struct NewReview {
    pub review_title:  String,
    pub review_body:   String,
    pub product_id:    String,
    pub review_rating: i32,
    pub vector:        Vec<f32>,
}

/// Response for single insert
#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub id:      u64,
    pub success: bool,
}

/// Response for bulk insert
#[derive(Debug, Serialize)]
pub struct BulkInsertResponse {
    pub inserted: usize,
    pub failed:   usize,
    pub errors:   Vec<String>,
}

/// Request payload for search
#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub vector: Vec<f32>,
    pub top_k:  Option<usize>,
}

/// One search hit in the response
#[derive(Debug, Serialize)]
pub struct SearchHit {
    pub id:     u64,
    pub score:  f32,
    pub review: ReviewMeta,
}
/// Single insert (already in your code)
/// POST /reviews
pub async fn insert_review(
    // Extension(store): Extension<Arc<Store>>,
    // Extension(embedder): Extension<Embedder>,
    // Extension(index): Extension<Arc<Index>>,
      State(state): State<AppState>,
    Json(payload): Json<NewReview>,
) -> Result<(StatusCode, Json<ApiResponse>), StatusCode> {
      // Simple validation
    if payload.review_title.trim().is_empty()
        || payload.review_body.trim().is_empty()
        || payload.product_id.trim().is_empty()
    {
        return Err(StatusCode::BAD_REQUEST);
    }
       // Generate  embed
       let meta = ReviewMeta {
        review_title:  payload.review_title,
        review_body:   payload.review_body,
        product_id:    payload.product_id,
        review_rating: payload.review_rating,
    };

   // Acquire global lock so vector+meta IDs stay in sync
    let _guard = state.commit_lock.lock().unwrap();

    // Append to vector index
    let mut idx = state.index.lock().unwrap();
    let id_idx = idx.append(&payload.vector)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Append to metadata store
    let id_meta = state.meta.append(&meta)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if id_idx != id_meta {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse { id: id_idx, success: true }),
    ))
}


/// POST /reviews/bulk
pub async fn insert_bulk_reviews(
    // Extension(store): Extension<Arc<Store>>,
    // Extension(embedder): Extension<Embedder>,
    // Extension(index): Extension<Arc<Index>>,
    State(state): State<AppState>,
    Json(items): Json<Vec<NewReview>>,
) -> Result<Json<BulkInsertResponse>, StatusCode> {
    let mut inserted = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    for (i, item) in items.into_iter().enumerate() {
        // 1. Validate non-empty
        if item.review_title.trim().is_empty()
            || item.review_body.trim().is_empty()
            || item.product_id.trim().is_empty()
        {
            failed += 1;
            errors.push(format!("item {}: validation failed", i));
            continue;
        }
        // Build meta & vector
         let meta = ReviewMeta {
            review_title:  item.review_title,
            review_body:   item.review_body,
            product_id:    item.product_id,
            review_rating: item.review_rating,
        };
        // lock vector+meta as a unit
        let _guard = state.commit_lock.lock().unwrap();
        let mut idx = state.index.lock().unwrap();

        match idx.append(&item.vector) {
            Ok(id_idx) => {
                match state.meta.append(&meta) {
                    Ok(id_meta) if id_meta == id_idx => {
                        inserted += 1;
                    }
                    Ok(_) => {
                        failed += 1;
                        errors.push(format!("item {}: ID mismatch", i));
                    }
                    Err(e) => {
                        failed += 1;
                        errors.push(format!("item {}: meta error {}", i, e));
                    }
                }
            }
            Err(e) => {
                failed += 1;
                errors.push(format!("item {}: index error {:?}", i, e));
            }
        }
    }

    Ok(Json(BulkInsertResponse {
        inserted,
        failed,
        errors,
    }))
}
// POST /search
pub async fn search_reviews(
    // Extension(store): Extension<Arc<Store>>,
    // Extension(embedder): Extension<Embedder>,
    // Extension(index): Extension<Arc<Index>>,
    State(state): State<AppState>,
    Json(req): Json<SearchRequest>,
) -> Result<Json<Vec<SearchHit>>, StatusCode> {
    let k = req.top_k.unwrap_or(5);

    // Search the vector index
    let idx = state.index.lock().unwrap();
    let hits = idx.search(&req.vector, k)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Map IDs back to metadata
    let mut out = Vec::with_capacity(hits.len());
    for (id, score) in hits {
        match state.meta.get(id) {
            Ok(review) => out.push(SearchHit { id, score, review }),
            Err(_)     => (), // skip missing
        }
    }

    Ok(Json(out))
}

