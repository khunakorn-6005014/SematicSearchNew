#ifndef SPFRESH_C_API_H
#define SPFRESH_C_API_H
 //backend/SPFresh/include/spfresh_c_api.h
#include <stdint.h>   // for int64_t

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handle to the SPFresh index
typedef void* SPFHandle;

// Success / error codes
#define SPF_OK                  0
#define SPF_ERROR_NULL_POINTER -1
#define SPF_ERROR_INIT_INDEX   -2
#define SPF_ERROR_ADD_VECTOR   -3
#define SPF_ERROR_SEARCH       -4

// Initialize (or open) an index on disk at `path`.
// On success returns SPF_OK and sets *out_handle.
// On failure returns a negative error code.
int spf_init_index(const char* path, SPFHandle* out_handle);

// Add a single embedding vector of dimension `dim`.
// Returns SPF_OK, or negative error.
int spf_add_vector(SPFHandle handle,
                   const float* vec,
                   int dim,
                   int64_t id);

// Query top-K nearest neighbors.
// `out_ids` must have space for at least `k` int64_t entries.
// `out_scores` must have space for at least `k` floats.
// Returns actual number of hits (0 ≤ return ≤ k), or negative on error.
int spf_search(SPFHandle handle,
               const float* query,
               int dim,
               int k,
               int64_t* out_ids,
               float* out_scores);

// Flush and free the index handle.
// Returns SPF_OK or negative error.
int spf_close(SPFHandle handle);

#ifdef __cplusplus
}
#endif

#endif // SPFRESH_C_API_H
