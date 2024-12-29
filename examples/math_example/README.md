# Bazel Math Example

This is an example project demonstrating how to use the `bazel_math` library, which provides C++-implemented math operations through Rust bindings.

## Running the Example

```bash
cargo run
```

## Expected Output

```
Basic Math Operations (a = 10.0, b = 5.0):
Addition (a + b): 15.0
Subtraction (a - b): 5.0
Multiplication (a * b): 50.0
Division (a / b): 2.0

Collatz Sequence Example:
7 → 22 → 11 → 34 → 17 → 52 → 26 → 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
```

## Code Explanation

The example demonstrates:
1. Basic arithmetic operations using the C++ implementation
2. The Collatz sequence calculation using multiple operations
3. Proper error handling and type safety

## Project Structure

```
.
├── Cargo.toml    # Rust package manifest
├── build.rs      # Build script for C++ compilation
└── src/
    └── main.rs   # Example code
```

## How it Works

The example uses Cargo's build script system to compile the C++ code and link it with the Rust binary. The `build.rs` script:
1. Compiles the C++ source files
2. Sets up the correct include paths
3. Links the resulting library with the Rust code

## License

MIT 