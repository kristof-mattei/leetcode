fn single_number(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, i| acc ^ i)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn single_number(nums: Vec<i32>) -> i32 {
        single_number(&nums)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0136::single_number;

    #[test]
    fn test_1() {
        assert_eq!(single_number(&[2, 2, 1]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(single_number(&[4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(single_number(&[1]), 1);
    }

    #[test]
    fn test_4() {
        // assert_eq!(single_number(&[2, 2, 1]), 1);
    }
}
