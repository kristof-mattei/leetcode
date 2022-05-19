fn max_coins(nums: &[i32]) -> i32 {
    let mut dp = vec![vec![0; nums.len()]; nums.len()];

    for g in 0..nums.len() {
        for (i, j) in (g..nums.len()).enumerate() {
            let mut max = i32::MIN;
            for k in i..=j {
                let left = if k == i { 0 } else { dp[i][k - 1] };
                let right = if k == j { 0 } else { dp[k + 1][j] };

                let left_after_pop = if i == 0 { 1 } else { nums[i - 1] };
                let right_after_pop = if j == nums.len() - 1 { 1 } else { nums[j + 1] };

                let cost = left_after_pop * nums[k] * right_after_pop;

                max = i32::max(max, left + right + cost);
            }

            dp[i][j] = max;
        }
    }

    dp[0][nums.len() - 1]
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        max_coins(&nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0312::max_coins;

    #[test]
    fn test_1() {
        assert_eq!(max_coins(&[3, 1, 5, 8]), 167);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_coins(&[1, 5]), 10);
    }
}
