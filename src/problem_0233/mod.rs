fn count_digit_one(n: i32) -> i32 {
    let mut res = 0;
    let mut i = 1;

    while i <= n {
        let prefix = n / (i * 10);
        let digit = (n / i) % 10;
        let suffix = n % i;

        if digit == 0 {
            res += prefix * i;
        } else if digit == 1 {
            res += prefix * i + suffix + 1;
        } else {
            res += (prefix + 1) * i;
        }

        i *= 10;
    }
    res
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn count_digit_one(n: i32) -> i32 {
        count_digit_one(n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0233::count_digit_one;

    #[test]
    fn test_1() {
        assert_eq!(count_digit_one(13), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(count_digit_one(0), 0);
    }
}
