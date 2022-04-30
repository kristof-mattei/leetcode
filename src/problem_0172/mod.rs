fn trailing_zeroes(n: i32) -> i32 {
    let mut count = 0;

    let mut i = 5;

    while n / i >= 1 {
        count += n / i;

        i *= 5;
    }

    count
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn trailing_zeroes(n: i32) -> i32 {
        trailing_zeroes(n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::trailing_zeroes;

    #[test]
    fn test_1() {
        assert_eq!(trailing_zeroes(3), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(trailing_zeroes(5), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(trailing_zeroes(0), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(trailing_zeroes(30), 7);
    }
}
