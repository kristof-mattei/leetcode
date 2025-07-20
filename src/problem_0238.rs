impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        product_except_self(&nums)
    }
}

fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let mut prefix = vec![0; nums.len()];
    let mut result = vec![0; nums.len()];

    prefix[0] = nums[0];

    for i in 1..nums.len() {
        prefix[i] = prefix[i - 1] * nums[i];
    }

    let last = nums.len() - 1;

    result[last] = prefix[last - 1];

    let mut suffix = nums[last];

    for i in (1..nums.len() - 1).rev() {
        result[i] = suffix * prefix[i - 1];
        suffix *= nums[i];
    }

    result[0] = suffix;

    result
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0238::product_except_self;

    #[test]
    fn test_1() {
        assert_eq!(product_except_self(&[1, 2, 3, 4]), [24, 12, 8, 6]);
    }

    #[test]
    fn test_2() {
        assert_eq!(product_except_self(&[-1, 1, 0, -3, 3]), [0, 0, 9, 0, 0]);
    }
}
