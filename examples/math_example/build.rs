fn main() {
    cc::Build::new()
        .cpp(true)
        .file("../../src/math/math.cc")
        .include("../../src")
        .compile("math");
    
    println!("cargo:rerun-if-changed=../../src/math/math.cc");
    println!("cargo:rerun-if-changed=../../src/math/math.h");
} 