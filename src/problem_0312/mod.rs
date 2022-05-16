fn max_coins(nums: &[i32]) -> i32 {
    0
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        max_coins(&nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0312::max_coins;

    #[test]
    fn test_1() {
        assert_eq!(max_coins(&[3, 1, 5, 8]), 167);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_coins(&[1, 5]), 10);
    }
}
