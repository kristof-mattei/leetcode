use std::{ops::Neg, vec::Vec};

#[derive(Clone, Copy)]
enum Direction {
    Decrease,
    Standstill,
    Increase,
}

impl std::ops::Add<Direction> for usize {
    type Output = usize;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            | Direction::Decrease => self.wrapping_sub(1),
            | Direction::Standstill => self,
            | Direction::Increase => self + 1,
        }
    }
}

impl std::ops::AddAssign<Direction> for usize {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            | Direction::Decrease => Direction::Increase,
            | Direction::Standstill => Direction::Standstill,
            | Direction::Increase => Direction::Decrease,
        }
    }
}

fn spiral_order(matrix: &[Vec<i32>]) -> Vec<i32> {
    let height = matrix.len();
    let width = matrix.get(0).map_or(0, Vec::len);
    let mut found = vec![vec![false; width]; height];

    let mut results = Vec::new();

    let mut row: usize = 0;
    let mut col: usize = 0;

    let mut di = Direction::Standstill;
    let mut dj = Direction::Increase;

    for _ in 0..height * width {
        results.push(matrix[row][col]);
        found[row][col] = true;

        // turn 90 degrees
        if found[(row + di) % height][(col + dj) % width] {
            std::mem::swap(&mut di, &mut dj);
            dj = -dj;
        }

        row += di;
        col += dj;
    }

    results
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        spiral_order(&matrix)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0054::spiral_order;

    #[test]
    fn test_1() {
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];

        let result = spiral_order(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];

        let result = spiral_order(&[vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let expected = vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10];

        let result = spiral_order(&[
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_4() {
        let expected = vec![1, 2, 3, 6, 9, 12, 11, 10, 7, 4, 5, 8];

        let result = spiral_order(&[
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_5() {
        let expected = vec![2, 5, 8, -1, 0, 4];

        let result = spiral_order(&[vec![2, 5, 8], vec![4, 0, -1]]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_6() {
        let expected = vec![1, 2, 4, 6, 5, 3];

        let result = spiral_order(&[vec![1, 2], vec![3, 4], vec![5, 6]]);

        assert_eq!(result, expected);
    }
}
