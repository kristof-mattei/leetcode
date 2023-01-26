fn min_sub_array_len(target: i32, nums: &[i32]) -> i32 {
    let mut result = nums.len();

    let mut sum = 0;
    let mut start = 0;

    // slide from left to right
    for end in 0..nums.len() {
        sum += nums[end];

        while sum >= target {
            result = result.min(end - start);
            sum -= nums[start];
            start += 1;
        }
    }

    if result == nums.len() {
        0
    } else {
        result as i32 + 1
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        min_sub_array_len(target, &nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0209::min_sub_array_len;

    #[test]
    fn test_0() {
        assert_eq!(min_sub_array_len(99, &[2, 3, 5, 9, 7, 3]), 0);
    }

    #[test]
    fn test_1() {
        assert_eq!(min_sub_array_len(7, &[2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(min_sub_array_len(4, &[1, 4, 4]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(min_sub_array_len(11, &[1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(min_sub_array_len(11, &[1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn test_5() {
        assert_eq!(
            min_sub_array_len(10, &[5, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            2
        );
    }
}
