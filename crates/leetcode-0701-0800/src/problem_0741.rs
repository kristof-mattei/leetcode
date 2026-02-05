impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        cherry_pickup(&grid)
    }
}

fn cherry_pickup(grid: &[Vec<i32>]) -> i32 {
    let n = grid.len();

    let mut dp = vec![vec![vec![i32::MIN; n]; n]; n];

    dp[0][0][0] = grid[0][0];

    for x1 in 0..n {
        for y1 in 0..n {
            for x2 in 0..n {
                let Some(y2) = (x1 + y1).checked_sub(x2) else {
                    continue;
                };

                if y2 >= n {
                    continue;
                }

                if grid[x1][y1] == -1 || grid[x2][y2] == -1 {
                    continue;
                }
                if x1 == 0 && y1 == 0 && x2 == 0 {
                    // base case already set
                    continue;
                }

                let mut val = i32::MIN;

                for (dx1, dy1) in [(0, 1), (1, 0)] {
                    for (dx2, dy2) in [(0, 1), (1, 0)] {
                        if let (Some(px1), Some(py1), Some(px2), Some(py2)) = (
                            x1.checked_sub(dx1),
                            y1.checked_sub(dy1),
                            x2.checked_sub(dx2),
                            y2.checked_sub(dy2),
                        ) {
                            if px1 < n && py1 < n && px2 < n && py2 < n {
                                val = val.max(dp[px1][py1][px2]);
                            }
                        }
                    }
                }

                if val == i32::MIN {
                    continue;
                }

                let mut cherries = grid[x1][y1];

                if x1 != x2 || y1 != y2 {
                    cherries += grid[x2][y2];
                }

                dp[x1][y1][x2] = val + cherries;
            }
        }
    }

    dp[n - 1][n - 1][n - 1].max(0)
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0741::cherry_pickup;

    #[test]
    fn test_1() {
        assert_eq!(
            cherry_pickup(
                &[[0, 1, -1], [1, 0, -1], [1, 1, 1]]
                    .into_iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
            ),
            5
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            cherry_pickup(
                &[[1, 1, -1], [1, -1, 1], [-1, 1, 1]]
                    .into_iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
            ),
            0
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            cherry_pickup(
                &[[1]]
                    .into_iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
            ),
            1
        );
    }

    #[test]
    fn test_4() {
        let input = [
            [1, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 1],
            [1, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1],
        ];

        assert_eq!(
            cherry_pickup(
                &input
                    .into_iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
            ),
            15
        );
    }
}
