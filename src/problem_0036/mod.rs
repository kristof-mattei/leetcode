use std::collections::HashSet;

fn is_valid_sudoku(board: &[Vec<char>]) -> bool {
    let mut h = HashSet::<char>::with_capacity(9);
    // rows
    for row in board {
        h.clear();

        if row.iter().any(|c| !check_hashmap(&mut h, *c)) {
            return false;
        }
    }

    // columns
    for c in 0..9 {
        h.clear();

        if board
            .iter()
            .map(|row| row[c])
            .any(|c| !check_hashmap(&mut h, c))
        {
            return false;
        }
    }

    // quadrants
    let left_top = [
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ];

    for row_quadrant in 0..=2 {
        for col_quadrant in 0..=2 {
            h.clear();

            if left_top
                .iter()
                .map(|(row, col)| board[row + (3 * row_quadrant)][col + (3 * col_quadrant)])
                .any(|c| !check_hashmap(&mut h, c))
            {
                return false;
            }
        }
    }

    true
}

fn check_hashmap(hash_map: &mut HashSet<char>, c: char) -> bool {
    if c == '.' {
        return true;
    }

    hash_map.insert(c)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        is_valid_sudoku(&board)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0036::is_valid_sudoku;

    #[test]
    fn test() {
        let board = [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(is_valid_sudoku(
            board
                .iter()
                .map(|l| l.to_vec())
                .collect::<Vec<_>>()
                .as_ref()
        ));
    }

    #[test]
    fn test_2() {
        let board = [
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(!is_valid_sudoku(
            board
                .iter()
                .map(|l| l.to_vec())
                .collect::<Vec<_>>()
                .as_ref()
        ));
    }

    #[test]
    fn test_3() {
        let board = [
            ['.', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '8', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(!is_valid_sudoku(
            board
                .iter()
                .map(|l| l.to_vec())
                .collect::<Vec<_>>()
                .as_ref()
        ));
    }
}
