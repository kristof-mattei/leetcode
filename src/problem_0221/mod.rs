fn maximal_square(matrix: &[Vec<char>]) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    if matrix.is_empty() {
        return 0;
    }
    let mut dp = vec![vec![0; cols + 1]; rows + 1];
    let mut max = 0;

    for r in 1..=rows {
        for c in 1..=cols {
            if matrix[r - 1][c - 1] == '1' {
                let left_up = dp[r - 1][c - 1];
                let up = dp[r - 1][c];
                let left = dp[r][c - 1];

                // plus one because... we're one!
                dp[r][c] = [left_up, up, left].iter().min().unwrap() + 1;

                max = i32::max(max, dp[r][c]);
            }
        }
    }

    max * max
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        maximal_square(&matrix)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0221::maximal_square;

    #[test]
    fn test_1() {
        let input = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];

        assert_eq!(maximal_square(&input), 4);
    }

    #[test]
    fn test_2() {
        let input = vec![vec!['0', '1'], vec!['1', '0']];

        assert_eq!(maximal_square(&input), 1);
    }

    #[test]
    fn test_3() {
        let input = vec![vec!['0']];

        assert_eq!(maximal_square(&input), 0);
    }
}
