use crate::shared::Solution;

fn jump(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return 0;
    }

    let mut index = 0;
    let mut jumps = 0;

    while index < nums.len() {
        let val = nums[index] as usize;

        let mut addition = (0, 0);

        // find the
        for index2 in (1..=val).rev() {
            if index + index2 == nums.len() - 1 {
                return jumps + 1;
            }
            if let Some(&new) = nums.get(index + index2) {
                let maybe_addition = index2 + new as usize;

                if maybe_addition > addition.0 + addition.1 {
                    addition = (index2, new as usize);
                }
            }
        }

        jumps += 1;
        index += addition.0;
    }

    jumps
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn jump(nums: Vec<i32>) -> i32 {
        jump(&nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_45::jump;

    #[test]
    fn test_1() {
        assert_eq!(jump(&[2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(jump(&[2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(jump(&[0]), 0);
    }
}
