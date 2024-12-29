load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "bazel_math",
    srcs = ["lib.rs"],
    deps = ["//src/math:math"],
    edition = "2021",
    visibility = ["//visibility:public"],
)

rust_test(
    name = "bazel_math_test",
    crate = ":bazel_math",
    edition = "2021",
) 