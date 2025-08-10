
//backend/SPFresh/include/spfresh_c_api.h
#ifndef SPFRESH_C_API_H
#define SPFRESH_C_API_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
// Opaque handle to the SPFresh index
typedef void* SPFHandle;
// Initialize (or open) an index on disk at `path`.
// On success returns 0 and sets *out_handle; on failure returns non‐zero.
int spf_init_index(const char* path, SPFHandle* out_handle);
// Add a single embedding vector of dimension `dim`.
// `vec` points to `dim` consecutive floats, `id` is your record identifier.
// Returns 0 on success, non‐zero on error.
int spf_add_vector(SPFHandle handle,
                   const float* vec,
                   int dim,
                   int64_t id);
// Query top‐K nearest neighbors.
// `query` points to your query vector (length = dim).
// `out_ids` must have space for at least `k` int64_t entries.
// `out_scores` must have space for at least `k` floats.
// Returns actual number of hits (≤ k), or negative on error.
int spf_search(SPFHandle handle,
               const float* query,
               int dim,
               int k,
               int64_t* out_ids,
               float* out_scores);
// Flush any buffers and free the handle.
// Returns 0 on success.
int spf_close(SPFHandle handle);
#ifdef __cplusplus
}
#endif
#endif // SPFRESH_C_API_H
