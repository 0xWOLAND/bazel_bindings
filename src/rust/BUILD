load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "math_bindings",
    srcs = ["math_bindings.rs"],
    deps = ["//src/math:math"],
    edition = "2021",
    visibility = ["//visibility:public"],
)

rust_test(
    name = "math_bindings_test",
    crate = ":math_bindings",
    edition = "2021",
) 