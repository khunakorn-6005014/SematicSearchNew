// src/spfresh_c_api.cpp

#include "spfresh_c_api.h"
#include "spfresh/index.hpp"    // adjust this include path if needed

#include <vector>
#include <new>       // std::bad_alloc
#include <exception> // std::exception
#include <cstdint>   // for int64_t

using spfresh::Index;

// Initialize (or open) an index on disk at `path`.
// On success returns SPF_OK and sets *out_handle.
// On failure returns a negative error code.
int spf_init_index(const char* path, SPFHandle* out_handle) {
    if (!path || !out_handle) {
        return SPF_ERROR_NULL_POINTER;
    }
    try {
        Index* idx = new Index(path);
        if (!idx->good()) {
            delete idx;
            return SPF_ERROR_INIT_INDEX;
        }
        *out_handle = static_cast<SPFHandle>(idx);
        return SPF_OK;
    }
    catch (const std::bad_alloc&) {
        return SPF_ERROR_INIT_INDEX;
    }
    catch (...) {
        return SPF_ERROR_INIT_INDEX;
    }
}

// Add a single embedding vector of dimension `dim`.
// Returns SPF_OK, or negative error.
int spf_add_vector(SPFHandle handle,
                   const float* vec,
                   int dim,
                   int64_t id) {
    if (!handle || !vec) {
        return SPF_ERROR_NULL_POINTER;
    }
    if (dim <= 0) {
        return SPF_ERROR_ADD_VECTOR;
    }

    auto* idx = static_cast<Index*>(handle);
    try {
        std::vector<float> cpp_vec(vec, vec + dim);
        idx->add(cpp_vec, id);
        return SPF_OK;
    }
    catch (...) {
        return SPF_ERROR_ADD_VECTOR;
    }
}

// Query top-K nearest neighbors.
// Returns actual number of hits (0 ≤ return ≤ k), or negative on error.
int spf_search(SPFHandle handle,
               const float* query,
               int dim,
               int k,
               int64_t* out_ids,
               float* out_scores) {
    if (!handle || !query || !out_ids || !out_scores) {
        return SPF_ERROR_NULL_POINTER;
    }
    if (dim <= 0 || k <= 0) {
        return SPF_ERROR_SEARCH;
    }

    auto* idx = static_cast<Index*>(handle);
    try {
        std::vector<float> cpp_q(query, query + dim);
        auto results = idx->knn_search(cpp_q, k);
        int n = static_cast<int>(results.size());
        for (int i = 0; i < n; ++i) {
            out_ids[i]    = results[i].first;
            out_scores[i] = results[i].second;
        }
        return n;
    }
    catch (...) {
        return SPF_ERROR_SEARCH;
    }
}

// Flush any buffers and free the handle.
// Returns SPF_OK or negative error.
int spf_close(SPFHandle handle) {
    if (!handle) {
        return SPF_ERROR_NULL_POINTER;
    }

    auto* idx = static_cast<Index*>(handle);
    try {
        delete idx;
        return SPF_OK;
    }
    catch (...) {
        // delete should not throw, but guard defensively
        return SPF_ERROR_SEARCH;
    }
}
