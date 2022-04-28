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

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0050::my_pow, utils::nearly_equal};

    #[test]
    fn test_1() {
        let expected = 1024.00000;

        let result = my_pow(2.00000, 10);

        assert!(nearly_equal(result, expected, f64::EPSILON));
    }

    #[test]

    fn test_2() {
        let expected = 9.26100;

        let result = my_pow(2.10000, 3);

        assert!(nearly_equal(result, expected, f64::EPSILON));
    }

    #[test]
    fn test_3() {
        let expected = 0.25000;

        let result = my_pow(2.00000, -2);

        assert!(nearly_equal(result, expected, f64::EPSILON));
    }
}
