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
├── handlers/
│   └── reviews.rs
├── spfresh_ffi.rs
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
$env:BOOST_ROOT = "C:\local\boost_1_88_0"
$env:BOOST_INCLUDEDIR="$env:BOOST_ROOT\include"
$env:BOOST_LIBRARYDIR="$env:BOOST_ROOT\lib64-msvc-14.3"
$env:BOOST_LIBRARYDIR = "$env:BOOST_ROOT\stage\lib"
cargo clean
cargo update
cargo build	
 .\b2 --version  
 bootstrap.bat      
 cargo update -p axum                                             
>> cargo update -p smallvec
wsl --list --verbose 
 git submodule update --init --recursive 
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

or 

git clone https://github.com/microsoft/vcpkg
.\vcpkg\bootstrap-vcpkg.bat
.\vcpkg\vcpkg.exe install rocksdb:x64-windows
.\vcpkg.exe install zstd:x64-windows tbb:x64-windows
.\vcpkg.exe install boost-test:x64-windows


add in build.rs
.define("CMAKE_TOOLCHAIN_FILE", "C:/path/to/vcpkg/scripts/buildsystems/vcpkg.cmake")

# from SPFresh/ThirdParty/zstd/build/cmake
cmake_minimum_required(VERSION 3.5)

if you don’t need all the extras, can pass flags 
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DROCKSDB_BUILD_SHARED=OFF -DWITH_TESTS=OFF -DWITH_BENCHMARK_TOOLS=OFF -DWITH_TOOLS=OFF
or 
cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug -DROCKSDB_BUILD_SHARED=OFF
kljkl

sudo apt install cmake
sudo apt install libjemalloc-dev libsnappy-dev libgflags-dev
sudo apt install pkg-config
sudo apt install swig libboost-all-dev
sudo apt install libtbb-dev
sudo apt install libisal-dev

 cargo install trunk                                              
>> choco install strawberryperl -y    
perl --version    
cargo install cargo-leptos --locked    
rustup target add wasm32-unknown-unknown  
> Build SPFresh
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make -j