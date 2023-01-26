use std::mem;

fn rotate(nums: &mut [i32], k: i32) {
    let k: usize = k as usize % nums.len();

    let mut current_i = 0;
    let mut already_rotated = 0;

    while already_rotated < nums.len() {
        let mut start = current_i;
        let mut value_to_move = nums[start];
        loop {
            // % to ensure we loop around
            let target_i = (start + k) % nums.len();

            mem::swap(&mut nums[target_i], &mut value_to_move);

            already_rotated += 1;

            start = target_i;

            if start == current_i {
                break;
            }
        }

        current_i += 1;
    }
}

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        rotate(nums, k);
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0189::rotate;

    #[test]
    fn test_1() {
        let mut nums = [1, 2, 3, 4, 5, 6, 7];

        rotate(&mut nums, 3);
        assert_eq!(nums, [5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_2() {
        let mut nums = [-1, -100, 3, 99];

        rotate(&mut nums, 2);
        assert_eq!(nums, [3, 99, -1, -100]);
    }

    #[test]
    fn test_3() {
        let mut nums = [-1];

        rotate(&mut nums, 2);
        assert_eq!(nums, [-1]);
    }

    #[test]
    fn test_4() {
        let mut nums = [-1];

        rotate(&mut nums, -2);
        assert_eq!(nums, [-1]);
    }
}
