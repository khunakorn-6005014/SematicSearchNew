## Backend Overview
The backend is a Rust/Axum service that manages an append-only vector index () and metadata store (). It exposes:
• 	POST /reviews: Insert a single review
• 	POST /reviews/bulk: Insert multiple reviews in one call
• 	POST /search: Semantic search by query, returns top-K matches
• 	GET /health: Health check

- Handlers live in src/handlers/reviews.rs.
- main.rs only wires routes and extensions—no business logic.
- Shared state (Store, Embedder) is injected via Extension.
- Error types live in src/error.rs, so handlers can return Result<…, ApiError>.
- store.rs only needs a helper next_id(); everything else (append/search) stays there.

src/
├── main.rs
├── error.rs
├── handlers/
│   └── reviews.rs
└── store.rs

• Vector index 
    ○ Stored in data/reviews.index via spfresh
    ○ Append-only for new embeddings
• Metadata file 
    ○ Stored in data/reviews.jsonl (one JSON object per line)
    ○ Contains fields like review_title, review_body, product_id, review_rating
    • Mapping strategy 
        Relies on insertion order: the Nth vector in the index corresponds to the Nth line in the JSONL

## TEST
 -POST localhost:8080/reviews
 -POST localhost:8080/search

$env:BOOST_ROOT="C:\path\to\boost_1_88_0"
$env:BOOST_INCLUDEDIR="$env:BOOST_ROOT\include"
$env:BOOST_LIBRARYDIR="$env:BOOST_ROOT\lib64-msvc-14.3"
cargo clean
cargo update
cargo build	

Rocksdb
git clone https://github.com/PtilopsisL/rocksdb.git
# Create a separate build folder
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DROCKSDB_BUILD_SHARED=OFF

# Build it
cmake --build build --config Release -- /m

# Install it into a local folder (e.g., build\install)
cmake --install build --prefix build\install

Test-Path "C:\Users\user\Downloads\seimaticSeach\backend\SPFresh\rocksdb\build\install\lib\cmake\rocksdb\RocksDBConfig.cmake"

$env:RocksDB_DIR = "C:\Users\user\Downloads\seimaticSeach\backend\SPFresh\rocksdb\build\install\lib\cmake\rocksdb"

if you don’t need all the extras, can pass flags 
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DROCKSDB_BUILD_SHARED=OFF -DWITH_TESTS=OFF -DWITH_BENCHMARK_TOOLS=OFF -DWITH_TOOLS=OFF
or 
cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug -DROCKSDB_BUILD_SHARED=OFF
kljkl