//! Local `number` module for the `web` binary.
//!
//! Provide a small `gcd` implementation so `mod number;` in `src/web/main.rs`
//! can include this file and the handlers can call `gcd`.

/// Returns the greatest common divisor of `n` and `m` using Euclid's algorithm.
///
/// Both `n` and `m` must be non-zero.
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0, "gcd arguments must be non-zero");
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn gcd_of_coprime() {
        assert_eq!(gcd(14, 15), 1);
    }

    #[test]
    fn gcd_of_related_numbers() {
        assert_eq!(gcd(2 * 3 * 5 * 7, 3 * 7 * 11), 21);
    }
}
