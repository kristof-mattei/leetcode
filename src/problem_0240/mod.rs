impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        search_matrix(&matrix, target)
    }
}

fn search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    search_matrix_r(matrix, (0, 0), target)
}

fn search_matrix_r(matrix: &[Vec<i32>], coordinates: (usize, usize), target: i32) -> bool {
    let value = matrix
        .get(coordinates.0)
        .and_then(|row| row.get(coordinates.1));

    let Some(&value) = value else {
        return false;
    };

    if value == target {
        return true;
    }

    if value < target && coordinates.1 + 1 < matrix[0].len() {
        search_matrix_r(matrix, (coordinates.0, coordinates.1 + 1), target)
    } else {
        let row = coordinates.0;
        let mut column = coordinates.1;

        for r in row + 1..matrix.len() {
            for c in (0..=(column.saturating_sub(1))).rev() {
                let value = matrix.get(r).and_then(|row| row.get(c));

                let value = *value.unwrap();

                if value <= target {
                    return search_matrix_r(matrix, (r, c), target);
                }
            }

            column = coordinates.1;
        }

        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0240::search_matrix;

    #[test]
    fn test_1() {
        assert!(!search_matrix(
            &[
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            20
        ));
    }

    #[test]
    fn test_2() {
        assert!(search_matrix(
            &[
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            5
        ));
    }

    #[test]
    fn test_3() {
        assert!(search_matrix(
            &[
                vec![2, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            5
        ));
    }

    #[test]
    fn test_4() {
        assert!(search_matrix(
            &[
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25]
            ],
            15
        ));
    }

    #[test]
    fn test_5() {
        assert!(search_matrix(
            &[
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25]
            ],
            5
        ));
    }

    #[test]
    fn test_6() {
        let result = search_matrix(&[vec![5], vec![6]], 6);
        assert!(result);
    }
}
