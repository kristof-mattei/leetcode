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
            Ordering::Greater => {
                offset = mid + 1;
            },
            Ordering::Equal => {
                end -= 1;
            },
        }
    }

    nums[offset]
}

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn find_min_2(nums: Vec<i32>) -> i32 {
        find_min(&nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0154::find_min;

    #[test]
    fn test() {
        let input = vec![1, 3, 5];

        assert_eq!(find_min(&input), 1);
    }

    #[test]
    fn test_2() {
        let input = vec![2, 2, 2, 0, 1];

        assert_eq!(find_min(&input), 0);
    }

    #[test]
    fn test_3() {
        let input = vec![3, 1, 3];

        assert_eq!(find_min(&input), 1);
    }
}
