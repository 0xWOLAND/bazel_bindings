mod bindings {
    #[link(name = "math")]
    extern "C" {
        pub fn math_add(a: f64, b: f64) -> f64;
        pub fn math_sub(a: f64, b: f64) -> f64;
        pub fn math_mul(a: f64, b: f64) -> f64;
        pub fn math_div(a: f64, b: f64) -> f64;
    }
}

pub fn add(a: f64, b: f64) -> f64 {
    unsafe { bindings::math_add(a, b) }
}

pub fn sub(a: f64, b: f64) -> f64 {
    unsafe { bindings::math_sub(a, b) }
}

pub fn mul(a: f64, b: f64) -> f64 {
    unsafe { bindings::math_mul(a, b) }
}

pub fn div(a: f64, b: f64) -> f64 {
    unsafe { bindings::math_div(a, b) }
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
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(sub(5.0, 3.0), 2.0);
        assert_eq!(mul(4.0, 2.0), 8.0);
        assert_eq!(div(6.0, 2.0), 3.0);
    }

    #[test]
    fn test_collatz() {
        assert_eq!(collatz(4.0), 2.0);
        assert_eq!(collatz(5.0), 16.0);
        assert_eq!(collatz(1.0), 1.0);
    }
}