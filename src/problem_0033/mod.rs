use std::cmp::Ordering;

use crate::shared::Solution;

fn search(nums: &[i32], target: i32) -> i32 {
    let mut offset = 0;
    let mut end = nums.len();

    while offset < end {
        let middle = (end - offset) / 2 + offset;

        if nums[middle] == target {
            return middle as i32;
        }

        // if our middle > the end
        match nums[middle].cmp(&nums[end - 1]) {
            Ordering::Greater => {
                // if our target is below the middle
                // AND larger than the beginning
                if target >= nums[offset] && target < nums[middle] {
                    // then we search again, between offset and middle
                    end = middle;
                } else {
                    // our target is on the right side of middle, narrow down there
                    offset = middle + 1;
                }
            },
            Ordering::Less | Ordering::Equal => {
                // our middle is less than the end
                // if our target sits on the right side of middle
                if target > nums[middle] && target <= nums[end - 1] {
                    // we narrow down the scope
                    offset = middle + 1;
                } else {
                    // we look in offset..middle
                    end = middle;
                }
            },
        }
    }
    -1
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        search(&nums, target)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0033::search;

    #[test]
    fn test() {
        let input = vec![4, 5, 6, 7, 0, 1, 2];

        assert_eq!(search(&input, 0), 4);
    }

    #[test]
    fn test_2() {
        let input = vec![1];

        assert_eq!(search(&input, 0), -1);
    }

    #[test]
    fn test_3() {
        let input = vec![5, 6, 7, 8, 1, 2, 3, 4];

        assert_eq!(search(&input, 2), 5);
    }
}
