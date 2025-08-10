///backend/SPFresh/src/spfresh_c_api.cpp
#include "spfresh_c_api.h"
#include "spfresh/index.hpp"    // adjust path to the real SPFresh header
#include <vector>
#include <memory>
#include <algorithm>
#include <cstring>
using namespace spfresh;       // or your actual namespace
int spf_init_index(const char* path, SPFHandle* out_handle) {
    // Construct the C++ Index, owning the on‐disk storage
    Index* idx = new Index(std::string(path));
    if (!idx || !idx->good()) {
        delete idx;
        return -1;
    }
    *out_handle = static_cast<SPFHandle>(idx);
    return 0;
}
int spf_add_vector(SPFHandle handle,
                   const float* vec,
                   int dim,
                   int64_t id) {
    if (!handle || !vec || dim <= 0) return -1;
    Index* idx = static_cast<Index*>(handle);
    std::vector<float> v(vec, vec + dim);
    // Assuming `add` is the SPFresh method to append a vector+id
    // Replace `add` with the correct SPFresh API call if different
    idx->add(v, id);
    return 0;
}
int spf_search(SPFHandle handle,
               const float* query,
               int dim,
               int k,
               int64_t* out_ids,
               float* out_scores) {
    if (!handle || !query || !out_ids || !out_scores || dim <= 0 || k <= 0)
        return -1;
    Index* idx = static_cast<Index*>(handle);
    // Assuming `knn_search` returns vector<pair<id,score>>
    auto hits = idx->knn_search(std::vector<float>(query, query + dim), k);
    int count = std::min<int>(hits.size(), k);
    for (int i = 0; i < count; i++) {
        out_ids[i]    = hits[i].first;
        out_scores[i] = hits[i].second;
    }
    return count;
}
int spf_close(SPFHandle handle) {
    if (!handle) return -1;
    Index* idx = static_cast<Index*>(handle);
    delete idx;
    return 0;
}
