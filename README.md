# Bazel Math - C++ Math Operations for Rust

A Rust crate that provides mathematical operations implemented in C++ and exposed through Rust bindings using Bazel as the build system.

## Project Structure

```
.
├── Cargo.toml         # Rust crate manifest
├── MODULE.bazel       # Bazel module configuration (Bzlmod)
├── WORKSPACE          # Bazel workspace configuration
├── src/
│   ├── math/          # C++ implementation
│   │   ├── BUILD      # Bazel build file for C++ library
│   │   ├── math.h     # C++ header file
│   │   └── math.cc    # C++ implementation
│   └── rust/          # Rust crate
│       ├── BUILD      # Bazel build file for Rust library
│       └── lib.rs     # Rust library implementation
```

## Features

The crate provides the following operations:
- Addition (`add`)
- Subtraction (`sub`)
- Multiplication (`mul`)
- Division (`div`)
- Collatz sequence calculation (`collatz`)

All operations use `f64` (double precision) numbers.

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
bazel test //src/rust:bazel_math_test
```

## Usage

### As a Bazel Dependency

1. Add the dependency to your `MODULE.bazel`:
```python
bazel_dep(name = "bazel_math", version = "0.1.0")
```

2. Add the dependency to your `BUILD` file:
```python
deps = ["@bazel_math//src/rust:bazel_math"]
```

3. Use in your Rust code:
```rust
use bazel_math::{add, sub, mul, div, collatz};

fn main() {
    let sum = add(5.0, 3.0);     // Returns 8.0
    let diff = sub(5.0, 3.0);    // Returns 2.0
    let product = mul(4.0, 2.0); // Returns 8.0
    let quotient = div(6.0, 2.0); // Returns 3.0
    let next = collatz(5.0);     // Returns 16.0
}
```

### As a Cargo Dependency

Add this to your `Cargo.toml`:
```toml
[dependencies]
bazel_math = "0.1.0"
```

## Documentation

Documentation with examples is available in the Rust source code. You can generate the documentation with:
```bash
cargo doc --no-deps --open
```

## License

MIT