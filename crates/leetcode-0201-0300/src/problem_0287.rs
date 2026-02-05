impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn find_duplicate(input: Vec<i32>) -> i32 {
        find_duplicate(&input)
    }
}

fn find_duplicate(nums: &[i32]) -> i32 {
    // Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.
    // 1 <= n <= 105
    // nums.length == n + 1
    // 1 <= nums[i] <= n
    let mut has_seen_number = nums.to_vec();

    for &num in nums {
        // All the integers in nums appear only once except for precisely one integer which appears two or more times.
        if has_seen_number[num as usize] == -1 {
            return num;
        }
        has_seen_number[num as usize] = -1;
    }

    -1
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0287::find_duplicate;

    #[test]
    fn test_1() {
        assert_eq!(find_duplicate(&[1, 3, 4, 2, 2]), 2);
    }
}
