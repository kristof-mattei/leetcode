impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        cherry_pickup(&grid)
    }
}

/// Solves the Cherry Pickup problem using dynamic programming.
///
/// Key insight: Instead of simulating one person going (0,0) -> (n-1,n-1) -> (0,0),
/// we simulate TWO people BOTH starting at (0,0) and BOTH moving to (n-1,n-1) simultaneously.
/// This is equivalent because:
/// - Path A->B->A can be split into A->B (person 1) and B->A (person 2, reversed = A->B)
/// - Both people move at the same "step", so after k steps, x1+y1 == x2+y2 == k
///
/// dp[x1][y1][x2] = maximum cherries collected when:
/// - Person 1 is at (x1, y1)
/// - Person 2 is at (x2, y2) where y2 = x1 + y1 - x2 (derived from the invariant)
///
/// Since both move together, we only need 3 dimensions (y2 is computed).
fn cherry_pickup(grid: &[Vec<i32>]) -> i32 {
    let n = grid.len();

    // dp[x1][y1][x2] = max cherries when person 1 at (x1,y1), person 2 at (x2, y2)
    // Initialize with i32::MIN to represent "unreachable" states
    let mut dp = vec![vec![vec![i32::MIN; n]; n]; n];

    // Base case: both people start at (0, 0)
    // They can only pick the cherry once even if both are on the same cell
    dp[0][0][0] = grid[0][0];

    // Iterate through all possible positions
    // x1, y1 define person 1's position
    // x2 defines person 2's x-coordinate (y2 is derived)
    for x1 in 0..n {
        for y1 in 0..n {
            for x2 in 0..n {
                // Skip base case - already initialized above
                if x1 == 0 && y1 == 0 && x2 == 0 {
                    continue;
                }

                // Derive y2 from the invariant: both people have taken the same number of steps
                // steps = x1 + y1 = x2 + y2, therefore y2 = x1 + y1 - x2
                // Use checked_sub to handle the case where x2 > x1 + y1 (invalid state)
                let Some(y2) = (x1 + y1).checked_sub(x2) else {
                    continue;
                };

                // y2 must be within grid bounds
                if y2 >= n {
                    continue;
                }

                // If either person is on a thorn (-1), this state is invalid
                if grid[x1][y1] == -1 || grid[x2][y2] == -1 {
                    continue;
                }

                // Find the maximum cherries from all possible previous states
                // Each person could have come from either:
                // - The cell above (moved down, so previous was x-1)
                // - The cell to the left (moved right, so previous was y-1)
                let mut past_cheries = i32::MIN;

                // (dx1, dy1) represents where person 1 came from
                // (0, 1) means they moved right (previous y was y1-1)
                // (1, 0) means they moved down (previous x was x1-1)
                for (dx1, dy1) in [(0, 1), (1, 0)] {
                    // (dx2, _) represents where person 2 came from
                    // no need to calculate dy2 as there is only 1 possible dy2
                    for dx2 in [0, 1] {
                        // Calculate previous positions using checked_sub to avoid underflow
                        if let Some(px1) = x1.checked_sub(dx1)
                            && let Some(py1) = y1.checked_sub(dy1)
                            && let Some(px2) = x2.checked_sub(dx2)
                        {
                            past_cheries = past_cheries.max(dp[px1][py1][px2]);
                        }
                    }
                }

                // If no valid previous state exists, skip this state
                if past_cheries == i32::MIN {
                    continue;
                }

                // Calculate cherries collected at current positions
                let mut cherries = grid[x1][y1];

                // If both people are on different cells, person 2 also collects a cherry
                // If they're on the same cell, the cherry is only counted once
                if (x1, y1) != (x2, y2) {
                    cherries += grid[x2][y2];
                }

                // This is the max from any combination until we're here
                dp[x1][y1][x2] = past_cheries + cherries;
            }
        }
    }

    // Return the answer: both people meet at (n-1, n-1)
    // Use max(0) because if no valid path exists, we return 0 (not negative)
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
