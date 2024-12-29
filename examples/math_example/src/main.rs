use bazel_math::{add, sub, mul, div, collatz};

fn print_operation(name: &str, result: f64) {
    println!("{}: {}", name, result);
}

fn main() {
    let a = 10.0;
    let b = 5.0;
    
    println!("Basic Math Operations (a = {}, b = {}):", a, b);
    print_operation("Addition (a + b)", add(a, b));
    print_operation("Subtraction (a - b)", sub(a, b));
    print_operation("Multiplication (a * b)", mul(a, b));
    print_operation("Division (a / b)", div(a, b));

    println!("\nCollatz Sequence Example:");
    let mut n = 7.0;
    print!("{}", n);
    while n > 1.0 {
        n = collatz(n);
        print!(" → {}", n);
    }
    println!();
} 