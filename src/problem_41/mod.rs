use crate::shared::Solution;

fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    for i in 0..n {
        while nums[i] > 0
            && nums[i] as usize <= n
            && nums[i] as usize != i + 1
            && nums[nums[i] as usize - 1] != nums[i]
        {
            let t = nums[i] as usize - 1;
            nums.swap(t, i);
        }
    }

    for (i, v) in nums.into_iter().enumerate() {
        if v != i as i32 + 1 {
            return i as i32 + 1;
        }
    }

    n as i32 + 1
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        first_missing_positive(nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_41::first_missing_positive;

    #[test]
    fn test_1() {
        let result = first_missing_positive(vec![1, 2, 0]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let result = first_missing_positive(vec![3, 4, -1, 1]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let result = first_missing_positive(vec![7, 8, 9, 11, 12]);
        assert_eq!(result, 1);
    }
}
