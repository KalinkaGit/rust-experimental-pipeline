/// Additionne deux entiers.
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Soustrait b de a.
pub fn subtract(a: i64, b: i64) -> i64 {
    a - b
}

/// Multiplie deux entiers.
pub fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

/// Divise a par b. Retourne None si b == 0.
pub fn divide(a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Retourne le n-ième nombre de Fibonacci (itératif).
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n {
                let tmp = a + b;
                a = b;
                b = tmp;
            }
            b
        }
    }
}

/// Vérifie si un nombre est premier.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    let mut i = 3u64;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 2;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- add ---
    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-4, -6), -10);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 42), 42);
    }

    // --- subtract ---
    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 3), 7);
    }

    #[test]
    fn test_subtract_negative_result() {
        assert_eq!(subtract(3, 10), -7);
    }

    // --- multiply ---
    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
    }

    #[test]
    fn test_multiply_by_zero() {
        assert_eq!(multiply(99, 0), 0);
    }

    // --- divide ---
    #[test]
    fn test_divide_normal() {
        assert_eq!(divide(10, 2), Some(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), None);
    }

    // --- fibonacci ---
    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_sequence() {
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (i, &val) in expected.iter().enumerate() {
            assert_eq!(fibonacci(i as u32), val, "fibonacci({i}) failed");
        }
    }

    // --- is_prime ---
    #[test]
    fn test_is_prime_small_primes() {
        for &p in &[2, 3, 5, 7, 11, 13, 17, 19, 23] {
            assert!(is_prime(p), "{p} should be prime");
        }
    }

    #[test]
    fn test_is_prime_not_prime() {
        for &n in &[0, 1, 4, 6, 8, 9, 10, 15, 25] {
            assert!(!is_prime(n), "{n} should not be prime");
        }
    }
}
