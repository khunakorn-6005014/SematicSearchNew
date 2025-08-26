// build.rs 
use std::{env, path::PathBuf};
use cmake::Config;
fn main() {
    // Location of your SPFresh CMake project (relative to this build.rs)
   // let spfresh_dir = PathBuf::from("SPFresh");
    // Set up cmake config
  //  let mut cfg = cmake::Config::new(&spfresh_dir);
//    let manifest = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
// let cfg = cmake::Config::new(&manifest.join("SPFresh")).profile("Release");
// let dst = cfg.build();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let spfresh_src = manifest_dir.join("SPFresh");
    let mut cfg = Config::new(&spfresh_src);
    cfg.profile("Release");
    // Only on Windows: use your vcpkg toolchain file
    if env::var("TARGET").unwrap().contains("windows") {
        cfg.define(
            "CMAKE_TOOLCHAIN_FILE",
            r"C:/local/vcpkg/scripts/buildsystems/vcpkg.cmake",
        );
    }
    // Turn off Linux-specific and GPU pieces
    //cfg.profile("Release"); //DCMAKE_BUILD_TYPE=Release ..


     // CMake places built libs under {dst}/lib by default
   let dst = cfg.build();
    //  cargo find compiled library and connect
   //let lib_dir = dst.join("lib");
    // CMake by default drops libs in {dst}/lib (or lib64)
    // 5) Determine where the archive actually lives
    let lib_name = "libspfresh_lib.a";

    // 5a) Out-of-source build dir: <dst>/lib
    let candidate1 = dst.join("lib").join(lib_name);

    // 5b) On-source fallback: SPFresh/Release
    let candidate2 = spfresh_src.join("Release").join(lib_name);

    let lib_dir = if candidate1.exists() {
        candidate1.parent().unwrap().to_path_buf()
    } else if candidate2.exists() {
        candidate2.parent().unwrap().to_path_buf()
    } else {
        panic!(
            "could not find {} in either:\n  {}\n  {}",
            lib_name,
            candidate1.display(),
            candidate2.display()
        );
    };

    println!("cargo:warning=linking with static library in `{}`", lib_dir.display());
   // tell Rust where to find libspfresh.a
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
  //  println!("cargo:rustc-link-lib=static=spfresh_lib");
   println!("cargo:rustc-link-lib=static=spfresh");
//sdf
    // if your C++ code uses the C++ standard library, link it too:
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=static=AnnService");
    // re-run build.rs if any of these C++ sources or headers change
    println!("cargo:rerun-if-changed=SPFresh/src/spfresh_c_api.cpp");
    println!("cargo:rerun-if-changed=SPFresh/include/spfresh_c_api.h");
    println!("cargo:rerun-if-changed=SPFresh/include/spfresh/index.hpp");
}


