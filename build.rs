use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    // Build the C++ library using Bazel
    let status = Command::new("bazel")
        .args(&["build", "//src/math:math"])
        .status()
        .expect("Failed to run bazel build");

    if !status.success() {
        panic!("Failed to build C++ library with Bazel");
    }

    // Get the Bazel output directory (bazel-bin)
    let bazel_bin = Command::new("bazel")
        .args(&["info", "bazel-bin"])
        .output()
        .expect("Failed to get bazel-bin path")
        .stdout;
    let bazel_bin = String::from_utf8(bazel_bin).unwrap().trim().to_string();

    // Tell cargo where to find the library
    let lib_path = PathBuf::from(bazel_bin).join("src/math");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=math");

    // Tell cargo to rebuild if the source files change
    println!("cargo:rerun-if-changed=src/math/math.cc");
    println!("cargo:rerun-if-changed=src/math/math.h");
    println!("cargo:rerun-if-changed=src/math/BUILD");
} 