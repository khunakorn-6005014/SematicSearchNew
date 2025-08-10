///backend/SPFresh/include/spfresh/index.hpp
#pragma once

#include <string>
#include <vector>
#include <utility>
#include <cstdint>

namespace spfresh {

class Index {
public:
    // opens or creates an index at disk path
    explicit Index(const std::string& path);

    // indicates whether the index was loaded/created successfully
    bool good() const;

    // add one vector (length = vec.size()) with doc ID
    void add(const std::vector<float>& vec, int64_t id);

    // return up to k best (id,score) pairs
    std::vector<std::pair<int64_t, float>>
    knn_search(const std::vector<float>& query, int k);

    // cleans up resources
    ~Index();
};

} // namespace spfresh