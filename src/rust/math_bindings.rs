#[link(name = "math")]
extern "C" {
    fn math_add(a: f64, b: f64) -> f64;
    fn math_sub(a: f64, b: f64) -> f64;
    fn math_mul(a: f64, b: f64) -> f64;
    fn math_div(a: f64, b: f64) -> f64;
}

pub fn add(a: f64, b: f64) -> f64 {
    unsafe { math_add(a, b) }
}

pub fn sub(a: f64, b: f64) -> f64 {
    unsafe { math_sub(a, b) }
}

pub fn mul(a: f64, b: f64) -> f64 {
    unsafe { math_mul(a, b) }
}

pub fn div(a: f64, b: f64) -> f64 {
    unsafe { math_div(a, b) }
}
pub fn collatz(n: f64) -> f64 {
    if n <= 1.0 {
        return n;
    }
    if n % 2.0 == 0.0 {
        div(n, 2.0)
    } else {
        add(mul(3.0, n), 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_math_operations() {
        println!("add(2.0, 3.0) = {}", add(2.0, 3.0));
        println!("sub(5.0, 3.0) = {}", sub(5.0, 3.0));
        println!("mul(4.0, 2.0) = {}", mul(4.0, 2.0));
        println!("div(6.0, 2.0) = {}", div(6.0, 2.0));
    }

    #[test]
    fn test_collatz() {
        println!("collatz(4.0) = {}", collatz(4.0));
        println!("collatz(5.0) = {}", collatz(5.0));
        println!("collatz(1.0) = {}", collatz(1.0));
        assert_eq!(collatz(4.0), 2.0);
        assert_eq!(collatz(5.0), 16.0);
        assert_eq!(collatz(1.0), 1.0);
    }
}