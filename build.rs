use std::{env, path::Path};

fn main() {
    let adlx = Path::new("libs/ADLX");

    if !adlx.exists() {
        panic!(
            "ADLX not found in libs/ADLX - Download it here: https://github.com/GPUOpen-LibrariesAndSDKs/ADLX under libs/ADLX."
        );
    }

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .file("cpp/ADLXWrapper.cpp")
        .file(adlx.join("SDK/ADLXHelper/Windows/Cpp/ADLXHelper.cpp"))
        .file(adlx.join("SDK/Platform/Windows/WinAPIs.cpp"))
        .include(adlx.join("SDK"))
        .flag_if_supported("/EHsc");

    build.compile("ADLXWrapper");

    println!(
        "cargo:rustc-link-search=native={}",
        env::var("OUT_DIR").unwrap()
    );
    println!("cargo:rerun-if-changed=cpp/ADLXWrapper.cpp");
    println!("cargo:rerun-if-changed=libs/ADLX/SDK/ADLXHelper/Windows/Cpp/ADLXHelper.cpp");
    println!("cargo:rerun-if-changed=libs/ADLX/SDK/Platform/Windows/WinAPIs.cpp");
}
