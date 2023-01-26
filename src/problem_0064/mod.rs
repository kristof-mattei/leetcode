fn min_path_sum(grid: &[Vec<i32>]) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    if rows == 0 || cols == 0 {
        return 0;
    }

    let mut min_sum_to_cell = vec![vec![0; cols]; rows];

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &v) in row.iter().enumerate() {
            let mut right = i32::MAX;
            let mut down = i32::MAX;

            // go right
            if col_index > 0 {
                right = v + min_sum_to_cell[row_index][col_index - 1];
            }

            // go down
            if row_index > 0 {
                down = v + min_sum_to_cell[row_index - 1][col_index];
            }

            min_sum_to_cell[row_index][col_index] = if row_index == 0 && col_index == 0 {
                v
            } else {
                right.min(down)
            }
        }
    }

    min_sum_to_cell[rows - 1][cols - 1]
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        min_path_sum(&grid)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0064::min_path_sum;

    #[test]
    fn test_1() {
        assert_eq!(
            min_path_sum(&[vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(min_path_sum(&[vec![1, 2, 3], vec![4, 5, 6]]), 12);
    }
}
