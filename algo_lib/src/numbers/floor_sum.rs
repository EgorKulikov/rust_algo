/// Computes sum_{i=0}^{n-1} floor((a*i + b) / m) in O(log min(a, m)).
///
/// Requirements: n >= 0, m >= 1.
/// a and b can be any non-negative values.
pub fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    assert!(n >= 0);
    assert!(m >= 1);
    if n == 0 {
        return 0;
    }
    let mut ans = 0i64;
    ans += n * (n - 1) / 2 * (a / m);
    a %= m;
    ans += n * (b / m);
    b %= m;
    let y_max = (a * (n - 1) + b) / m;
    if y_max == 0 {
        return ans;
    }
    ans += (n - 1) * y_max - floor_sum(y_max, a, m, m - b - 1);
    ans
}

#[cfg(test)]
mod test {
    use super::floor_sum;

    #[test]
    fn basic_cases() {
        // sum_{i=0}^{3} floor((6*i + 3) / 10) = floor(3/10) + floor(9/10) + floor(15/10) + floor(21/10)
        // = 0 + 0 + 1 + 2 = 3
        assert_eq!(floor_sum(4, 10, 6, 3), 3);
    }

    #[test]
    fn zero_n() {
        assert_eq!(floor_sum(0, 5, 3, 2), 0);
    }

    #[test]
    fn a_zero() {
        // sum_{i=0}^{4} floor(b / m) = 5 * floor(7/3) = 5 * 2 = 10
        assert_eq!(floor_sum(5, 3, 0, 7), 10);
    }

    #[test]
    fn b_zero() {
        // sum_{i=0}^{4} floor(2*i / 3) = 0 + 0 + 1 + 2 + 2 = 5
        assert_eq!(floor_sum(5, 3, 2, 0), 5);
    }

    #[test]
    fn large_values() {
        // From ATCoder Practice Contest C sample
        assert_eq!(floor_sum(6, 5, 4, 3), 13);
        assert_eq!(floor_sum(1, 1, 0, 0), 0);
        assert_eq!(floor_sum(31415, 92653, 58979, 32384), 314095480);
        assert_eq!(
            floor_sum(1000000000, 1000000000, 999999999, 999999999),
            499999999500000000
        );
    }
}
