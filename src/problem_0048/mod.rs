fn swap(matrix: &mut [Vec<i32>], r1c1: (usize, usize), r2c2: (usize, usize)) {
    match r1c1.0.cmp(&r2c2.0) {
        std::cmp::Ordering::Equal => {
            matrix[r1c1.0].swap(r1c1.1, r2c2.1);
        },
        std::cmp::Ordering::Less => {
            swap_ord(matrix, r1c1, r2c2);
        },
        std::cmp::Ordering::Greater => {
            swap_ord(matrix, r2c2, r1c1);
        },
    }
}

fn swap_ord(
    matrix: &mut [Vec<i32>],
    low_row_coordinates: (usize, usize),
    high_row_coordinates: (usize, usize),
) {
    let (left, right) = matrix.split_at_mut(high_row_coordinates.0);
    std::mem::swap(
        &mut left[low_row_coordinates.0][low_row_coordinates.1],
        &mut right[0][high_row_coordinates.1],
    );
}

fn rotate(matrix: &mut [Vec<i32>]) {
    let n = matrix.len();

    for x in 0..n / 2 {
        for y in 0..(n + 1) / 2 {
            swap_ord(matrix, (x, y), (n - y - 1, x));

            swap(matrix, (n - y - 1, x), (n - x - 1, n - y - 1));

            swap_ord(matrix, (y, n - x - 1), (n - x - 1, n - y - 1));
        }
    }
}

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        rotate(matrix);
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0048::rotate;

    #[test]
    fn test_1() {
        let mut input = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];

        let expected = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];

        rotate(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_2() {
        let mut input = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];

        let expected = vec![
            vec![21, 16, 11, 6, 1],
            vec![22, 17, 12, 7, 2],
            vec![23, 18, 13, 8, 3],
            vec![24, 19, 14, 9, 4],
            vec![25, 20, 15, 10, 5],
        ];

        rotate(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_3() {
        let mut input = vec![vec![1]];

        let expected = vec![vec![1]];

        rotate(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_4() {
        let mut input = vec![vec![1, 2], vec![3, 4]];

        let expected = vec![vec![3, 1], vec![4, 2]];

        rotate(&mut input);
        assert_eq!(input, expected);
    }
}
