use std::cmp::Ordering;

fn search(nums: &[i32], target: i32) -> bool {
    let mut offset = 0;
    let mut end = nums.len();

    while offset < end {
        let middle = (end - offset) / 2 + offset;

        if nums[middle] == target {
            return true;
        }

        match nums[middle].cmp(&nums[end - 1]) {
            Ordering::Greater => {
                if target >= nums[offset] && target < nums[middle] {
                    end -= 1;
                } else {
                    offset = middle + 1;
                }
            },
            Ordering::Less => {
                if target > nums[middle] && target <= nums[end - 1] {
                    offset = middle + 1;
                } else {
                    end -= 1;
                }
            },
            Ordering::Equal => {
                while offset < nums.len() && nums[offset] == nums[middle] {
                    offset += 1;
                }
                while end > 0 && nums[end - 1] == nums[middle] {
                    end -= 1;
                }
            },
        }
    }
    false
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        search(&nums, target)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0081::search;

    #[test]
    fn test_1() {
        let input = vec![2, 5, 6, 0, 0, 1, 2];

        assert!(search(&input, 0));
    }

    #[test]
    fn test_2() {
        let input = vec![2, 5, 6, 0, 0, 1, 2];

        assert!(!search(&input, 3));
    }

    #[test]
    fn test_3() {
        let input = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1];

        assert!(search(&input, 2));
    }
}
