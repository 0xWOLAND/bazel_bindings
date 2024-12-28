# Bazel C++ and Rust Bindings Example

This project demonstrates how to create C++ libraries that can be called from Rust using Bazel as the build system. It includes a simple math library implemented in C++ and exposed to Rust through FFI bindings.

## Project Structure

```
.
├── MODULE.bazel       # Bazel module configuration (Bzlmod)
├── WORKSPACE          # Bazel workspace configuration
├── src/
│   ├── math/          # C++ implementation
│   │   ├── BUILD      # Bazel build file for C++ library
│   │   ├── math.h     # C++ header file
│   │   └── math.cc    # C++ implementation
│   └── rust/          # Rust bindings
│       ├── BUILD      # Bazel build file for Rust library
│       └── math_bindings.rs  # Rust FFI bindings
```

## Features

The math library provides the following operations:
- Addition (`add`)
- Subtraction (`sub`)
- Multiplication (`mul`)
- Division (`div`)
- Collatz sequence calculation (`collatz`)

## Prerequisites

- Bazel (latest version with Bzlmod support)
- Rust toolchain
- C++ compiler

## Building

To build the entire project:
```bash
bazel build //...
```

## Testing

To run the tests:
```bash
bazel test //src/rust:math_bindings_test
```

## Usage

To use the math library in your Rust code:

```rust
use math_bindings::{add, sub, mul, div, collatz};

fn main() {
    let sum = add(5.0, 3.0);     // Returns 8.0
    let diff = sub(5.0, 3.0);    // Returns 2.0
    let product = mul(4.0, 2.0); // Returns 8.0
    let quotient = div(6.0, 2.0); // Returns 3.0
    let next = collatz(5.0);     // Returns 16.0
}
```

## Adding to Your Project

To use this library in your Bazel project:

1. Add the dependency to your `MODULE.bazel`:
```python
bazel_dep(name = "bazel_bindings", version = "0.1.0")
```

2. Add the dependency to your `BUILD` file:
```python
deps = ["@bazel_bindings//src/rust:math_bindings"]
```

## License

[Insert your chosen license here]