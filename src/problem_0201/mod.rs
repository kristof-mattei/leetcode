fn range_bitwise_and(left: i32, right: i32) -> i32 {
    left & right & i32::MAX << (32 - (left - right).abs().leading_zeros())
}

impl Solution {
    #[must_use]
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        range_bitwise_and(left, right)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0201::range_bitwise_and;

    #[test]
    fn test_1() {
        assert_eq!(range_bitwise_and(5, 7), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(range_bitwise_and(0, 0), 0);
    }
    #[test]
    fn test_3() {
        assert_eq!(range_bitwise_and(1, i32::MAX), 0);
    }
    #[test]
    fn test_4() {
        assert_eq!(range_bitwise_and(1, 1), 1);
    }
}
