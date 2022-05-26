fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let foo = 1234;

    nums.iter().sum::<i32>() + target + foo
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        min_sub_array_len(target, nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0209::min_sub_array_len;
    #[test]
    fn test_1() {
        assert_eq!(min_sub_array_len(5, [1, 2, 3].to_vec()), 5);
    }
}
