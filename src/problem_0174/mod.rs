fn calculate_minimum_hp(dungeon: &[Vec<i32>]) -> i32 {
    let m = dungeon.len();
    let n = dungeon[0].len();

    let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];

    dp[m - 1][n] = 1;
    dp[m][n - 1] = 1;

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[i][j] = 1i32.max(dp[i + 1][j].min(dp[i][j + 1]) - dungeon[i][j]);
        }
    }

    dp[0][0]
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn calculate_minimum_hp(grid: Vec<Vec<i32>>) -> i32 {
        calculate_minimum_hp(&grid)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0174::calculate_minimum_hp;

    #[test]
    fn test_1() {
        assert_eq!(
            calculate_minimum_hp(&[vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]]),
            7
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(calculate_minimum_hp(&[vec![0]]), 1);
    }
}
