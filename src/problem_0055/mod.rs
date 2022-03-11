use crate::shared::Solution;

fn can_jump(nums: &[i32]) -> bool {
    let len = nums.len();

    let mut max_reach = 0;
    let mut i = 0;

    while i <= max_reach && i < len {
        max_reach = max_reach.max(i + nums[i] as usize);
        i += 1;
    }

    max_reach >= len - 1
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        can_jump(&nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0055::can_jump;

    #[test]
    fn test_1() {
        assert!(can_jump(&[2, 3, 1, 1, 4]));
    }

    #[test]
    fn test_2() {
        assert!(!can_jump(&[3, 2, 1, 0, 4]));
    }

    #[test]
    fn test_3() {
        assert!(can_jump(&[0]));
    }

    #[test]
    fn test_4() {
        assert!(can_jump(&[2, 0]));
    }

    #[test]
    fn test_5() {
        assert!(!can_jump(&[
            2, 0, 6, 9, 8, 4, 5, 0, 8, 9, 1, 2, 9, 6, 8, 8, 0, 6, 3, 1, 2, 2, 1, 2, 6, 5, 3, 1, 2,
            2, 6, 4, 2, 4, 3, 0, 0, 0, 3, 8, 2, 4, 0, 1, 2, 0, 1, 4, 6, 5, 8, 0, 7, 9, 3, 4, 6, 6,
            5, 8, 9, 3, 4, 3, 7, 0, 4, 9, 0, 9, 8, 4, 3, 0, 7, 7, 1, 9, 1, 9, 4, 9, 0, 1, 9, 5, 7,
            7, 1, 5, 8, 2, 8, 2, 6, 8, 2, 2, 7, 5, 1, 7, 9, 6
        ]));
    }
}
