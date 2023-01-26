use std::collections::HashSet;

fn set_zeroes(matrix: &mut [Vec<i32>]) {
    let mut rows_to_zero = HashSet::new();
    let mut cols_to_zero = HashSet::new();

    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            if value == 0 {
                rows_to_zero.insert(row_index);
                cols_to_zero.insert(col_index);
            }
        }
    }

    for row in rows_to_zero {
        // zero out row
        matrix[row].iter_mut().for_each(|v| *v = 0);
    }
    for col in cols_to_zero {
        // zero out col
        matrix.iter_mut().for_each(|r| r[col] = 0);
    }
}

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        set_zeroes(matrix);
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0073::set_zeroes;

    #[test]
    fn test_1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];

        set_zeroes(&mut matrix);
        assert_eq!(matrix, [[1, 0, 1], [0, 0, 0], [1, 0, 1]]);
    }

    #[test]
    fn test_2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];

        set_zeroes(&mut matrix);
        assert_eq!(matrix, [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);
    }
}
