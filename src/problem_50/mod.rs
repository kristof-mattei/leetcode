use crate::shared::Solution;

fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let t = my_pow(x, n / 2);

    if n % 2 == 0 {
        t * t
    } else if n > 0 {
        x * t * t
    } else {
        t * t / x
    }
}

impl Solution {
    #[must_use]
    pub fn my_pow(x: f64, n: i32) -> f64 {
        my_pow(x, n)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_50::my_pow;
    fn nearly_equal(mut a: f64, mut b: f64, epsilon: f64) -> bool {
        a = (a).abs();
        b = (b).abs();
        let diff = (a - b).abs();

        #[allow(clippy::float_cmp)]
        if a == b {
            // shortcut, handles infinities
            true
        } else if a == 0f64 || b == 0f64 || (a + b < f64::MIN_POSITIVE) {
            // a or b is zero or both are extremely close to it
            // relative error is less meaningful here
            diff < (epsilon * f64::MIN_POSITIVE)
        } else {
            // use relative error
            (diff / f64::min(a + b, f64::MAX)) < epsilon
        }
    }

    #[test]
    fn test_1() {
        assert!((my_pow(2.00000, 10) - 1024.00000).abs() < f64::EPSILON);
    }

    #[test]

    fn test_2() {
        let expected = 9.26100;

        let result = my_pow(2.10000, 3);

        assert!(nearly_equal(result, expected, f64::EPSILON));
    }

    #[test]
    fn test_3() {
        assert!((my_pow(2.00000, -2) - 0.25000).abs() < f64::EPSILON);
    }
}
