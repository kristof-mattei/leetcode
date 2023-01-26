use std::cmp::Ordering;

fn find_peak_element(nums: &[i32]) -> i32 {
    let mut offset = 0;
    let mut end = nums.len() - 1;

    while offset < end {
        let mid = offset + (end - offset) / 2;

        match nums[mid].cmp(&nums[mid + 1]) {
            Ordering::Less => {
                offset = mid + 1;
            },
            Ordering::Greater | Ordering::Equal => {
                end = mid;
            },
        }
    }

    offset as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        find_peak_element(&nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0162::find_peak_element;

    #[test]
    fn test() {
        let input = vec![1, 2, 3, 1];

        assert_eq!(find_peak_element(&input), 2);
    }

    #[test]
    fn test_2() {
        let input = vec![1, 2, 1, 3, 5, 6, 4];

        assert_eq!(find_peak_element(&input), 5);
    }
}
