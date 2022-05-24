fn visit_island(row_index: usize, col_index: usize, grid: &mut Vec<Vec<char>>) -> bool {
    if grid[row_index][col_index] == '1' {
        grid[row_index][col_index] = '0';

        if row_index > 0 {
            visit_island(row_index - 1, col_index, grid);
        }

        if col_index > 0 {
            visit_island(row_index, col_index - 1, grid);
        }

        if row_index + 1 < grid.len() {
            visit_island(row_index + 1, col_index, grid);
        }

        if col_index + 1 < grid[0].len() {
            visit_island(row_index, col_index + 1, grid);
        }

        return true;
    }

    false
}

fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for row_index in 0..grid.len() {
        for col_index in 0..grid[row_index].len() {
            if visit_island(row_index, col_index, grid) {
                count += 1;
            }
        }
    }

    count
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        num_islands(&mut grid)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::num_islands;

    #[test]
    fn test_1() {
        assert_eq!(
            num_islands(
                &mut [
                    ['1', '1', '1', '1', '0'].to_vec(),
                    ['1', '1', '0', '1', '0'].to_vec(),
                    ['1', '1', '0', '0', '0'].to_vec(),
                    ['0', '0', '0', '0', '0'].to_vec(),
                ]
                .to_vec()
            ),
            1
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            num_islands(
                &mut [
                    ['1', '1', '0', '0', '0'].to_vec(),
                    ['1', '1', '0', '0', '0'].to_vec(),
                    ['0', '0', '1', '0', '0'].to_vec(),
                    ['0', '0', '0', '1', '1'].to_vec(),
                ]
                .to_vec()
            ),
            3
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(num_islands(&mut [['1'].to_vec()].to_vec()), 1);
    }
}
