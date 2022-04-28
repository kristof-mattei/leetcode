use std::cmp::Ordering;

fn find_min(nums: &[i32]) -> i32 {
    let mut offset = 0;
    let mut end = nums.len() - 1;

    while offset < end {
        let mid = offset + (end - offset) / 2;

        match nums[mid].cmp(&nums[end]) {
            Ordering::Less => {
                end = mid;
            },
            Ordering::Greater | Ordering::Equal => {
                offset = mid + 1;
            },
            // see 0154
            // Ordering::Equal => {
            //     end -= 1;
            // },
        }
    }

    nums[offset]
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_min(nums: Vec<i32>) -> i32 {
        find_min(&nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0153::find_min;

    #[test]
    fn test() {
        let input = vec![3, 4, 5, 1, 2];

        assert_eq!(find_min(&input), 1);
    }

    #[test]
    fn test_2() {
        let input = vec![4, 5, 6, 7, 0, 1, 2];

        assert_eq!(find_min(&input), 0);
    }

    #[test]
    fn test_3() {
        let input = vec![11, 13, 15, 17];

        assert_eq!(find_min(&input), 11);
    }

    #[test]
    fn test_4() {
        let input = vec![1];

        assert_eq!(find_min(&input), 1);
    }

    #[test]
    fn test_5() {
        let input = vec![2, 1];

        assert_eq!(find_min(&input), 1);
    }
}
