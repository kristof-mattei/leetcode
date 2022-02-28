use std::collections::HashSet;

use crate::shared::Solution;

const QUADRANTS: [(usize, usize); 9] = [
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

struct World {
    board: Vec<Vec<char>>,
    guess_depth: usize,
}

fn check_hashmap(hash_map: &mut HashSet<char>, c: char) -> bool {
    if c == '.' {
        return true;
    }

    hash_map.insert(c)
}

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
    for row_quadrant in 0..=2 {
        for col_quadrant in 0..=2 {
            h.clear();

            if QUADRANTS
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

fn get_possibilities_in_quadrant(board: &[Vec<char>], row: usize, col: usize) -> HashSet<char> {
    // determine quadrant:
    let row_quadrant = row / 3;
    let col_quadrant = col / 3;

    QUADRANTS
        .iter()
        .map(|(row, col)| board[row + (3 * row_quadrant)][col + (3 * col_quadrant)])
        .filter(|c| c != &'.')
        .collect()
}

fn get_possibilities_in_row(board: &[Vec<char>], row: usize) -> HashSet<char> {
    board[row].iter().filter(|c| *c != &'.').copied().collect()
}

fn get_possibilities_in_col(board: &[Vec<char>], col: usize) -> HashSet<char> {
    board.iter().map(|v| v[col]).filter(|c| *c != '.').collect()
}

fn get_possibilities(board: &[Vec<char>], row: usize, col: usize) -> HashSet<char> {
    let nums_in_quadrant = get_possibilities_in_quadrant(board, row, col);
    let nums_in_row = get_possibilities_in_row(board, row);
    let nums_in_col = get_possibilities_in_col(board, col);

    let mut all_possibilities = ('1'..='9').collect::<HashSet<char>>();

    for c in vec![nums_in_col, nums_in_row, nums_in_quadrant]
        .iter()
        .flatten()
    {
        all_possibilities.remove(c);
    }

    all_possibilities
}

fn solve_sudoku_until_no_more_1(
    board: &mut Vec<Vec<char>>,
) -> Vec<((usize, usize), HashSet<char>)> {
    loop {
        let mut opens = vec![];
        let mut any_1 = false;
        for r in 0..=8 {
            for c in 0..=8 {
                if board[r][c] == '.' {
                    let p = get_possibilities(board, r, c);

                    if p.is_empty() {
                        return vec![];
                    }

                    if p.len() == 1 {
                        board[r][c] = *p.iter().next().unwrap();
                        any_1 = true;
                    } else {
                        opens.push(((r, c), p));
                    }
                }
            }
        }

        if any_1 {
        } else {
            return opens;
        }
    }
}

fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut worlds = vec![World {
        board: board.clone(),
        guess_depth: 1,
    }];

    loop {
        let board_index = worlds
            .iter()
            .enumerate()
            .min_by_key(|(_, w)| w.guess_depth)
            .map(|(i, ..)| i)
            .unwrap();

        let World {
            board: mut current_board,
            guess_depth,
        } = worlds.swap_remove(board_index);

        let opens = solve_sudoku_until_no_more_1(&mut current_board);

        if opens.is_empty() {
            if !current_board.iter().any(|r| r.iter().any(|c| c == &'.')) {
                for r in 0..=8 {
                    for c in 0..=8 {
                        board[r][c] = current_board[r][c];
                    }
                }

                return;
            }
            continue;
        }

        // get the ones with min possibilities
        let lowest_p = opens.iter().map(|((_, _), p)| p.len()).min().unwrap();
        for ((row, col), possibilities) in
            opens.into_iter().filter(|((_, _), p)| p.len() == lowest_p)
        {
            for p in possibilities {
                let mut new_board = current_board.clone();
                new_board[row][col] = p;

                // we multiply the guess depth with our current amount of possibilities
                // if we have only 1 choice for this cell, then that world should have a higher
                // priority than when we have a cell with 7 possibilities that we're
                // splitting in 7 worlds and trying each version
                worlds.push(World {
                    board: new_board,
                    guess_depth: guess_depth * lowest_p,
                });
            }
        }
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        solve_sudoku(board);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::problem_37::{
        get_possibilities, get_possibilities_in_col, get_possibilities_in_quadrant,
        get_possibilities_in_row, solve_sudoku,
    };

    fn get_board() -> Vec<Vec<char>> {
        let board = [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ]
        .iter()
        .map(|l| l.iter().map(|l| l.chars().next().unwrap()).collect())
        .collect::<Vec<_>>();

        board
    }

    #[test]
    fn test_row() {
        let board = get_board();
        let taken_numbers_in_row = get_possibilities_in_row(&board, 7);

        let row_numbers = HashSet::from(['4', '1', '9', '5']);
        assert_eq!(taken_numbers_in_row, row_numbers);
    }

    #[test]
    fn test_col() {
        let board = get_board();

        let taken_numbers_in_col = get_possibilities_in_col(&board, 3);

        let col_numbers = HashSet::from(['1', '8', '4']);
        assert_eq!(taken_numbers_in_col, col_numbers);
    }

    #[test]
    fn test_quadrant_1() {
        let board = get_board();
        let taken_numbers_in_quadrant = get_possibilities_in_quadrant(&board, 0, 0);

        let quadrant_numbers = HashSet::from(['5', '3', '6', '9', '8']);
        assert_eq!(taken_numbers_in_quadrant, quadrant_numbers);

        let taken_numbers_in_quadrant = get_possibilities_in_quadrant(&board, 2, 2);

        assert_eq!(taken_numbers_in_quadrant, quadrant_numbers);
    }

    #[test]
    fn test_quadrant_2() {
        let board = get_board();
        let taken_numbers_in_quadrant = get_possibilities_in_quadrant(&board, 6, 6);

        let quadrant_numbers = HashSet::from(['2', '8', '5', '7', '9']);
        assert_eq!(taken_numbers_in_quadrant, quadrant_numbers);

        let taken_numbers_in_quadrant = get_possibilities_in_quadrant(&board, 8, 8);

        assert_eq!(taken_numbers_in_quadrant, quadrant_numbers);
    }

    #[test]
    fn test_possibilities_1() {
        let board = get_board();
        let possibilities = get_possibilities(&board, 6, 5);

        assert_eq!(possibilities, HashSet::from(['7']));
    }

    #[test]
    fn test_possibilities_2() {
        let board = get_board();

        let possibilities = get_possibilities(&board, 2, 1);

        assert_eq!(possibilities, HashSet::from(['1', '2', '4', '7']));
    }
    #[test]
    fn test_possibilities_3() {
        let board = get_board();
        let possibilities = get_possibilities(&board, 5, 3);

        assert_eq!(possibilities, HashSet::from(['5', '9']));
    }

    #[test]
    fn test_1() {
        let mut board = get_board();

        let solution: Vec<Vec<char>> = [
            ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
            ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
            ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
            ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
            ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
            ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
            ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
            ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
            ["3", "4", "5", "2", "8", "6", "1", "7", "9"],
        ]
        .iter()
        .map(|l| l.iter().map(|l| l.chars().next().unwrap()).collect())
        .collect::<Vec<_>>();

        solve_sudoku(&mut board);

        assert_eq!(board, solution);
    }

    #[test]
    fn test_2() {
        let mut board = [
            [".", ".", "9", "7", "4", "8", ".", ".", "."],
            ["7", ".", ".", ".", ".", ".", ".", ".", "."],
            [".", "2", ".", "1", ".", "9", ".", ".", "."],
            [".", ".", "7", ".", ".", ".", "2", "4", "."],
            [".", "6", "4", ".", "1", ".", "5", "9", "."],
            [".", "9", "8", ".", ".", ".", "3", ".", "."],
            [".", ".", ".", "8", ".", "3", ".", "2", "."],
            [".", ".", ".", ".", ".", ".", ".", ".", "6"],
            [".", ".", ".", "2", "7", "5", "9", ".", "."],
        ]
        .iter()
        .map(|l| l.iter().map(|l| l.chars().next().unwrap()).collect())
        .collect::<Vec<_>>();

        let solution: Vec<Vec<char>> = [
            ["5", "1", "9", "7", "4", "8", "6", "3", "2"],
            ["7", "8", "3", "6", "5", "2", "4", "1", "9"],
            ["4", "2", "6", "1", "3", "9", "8", "7", "5"],
            ["3", "5", "7", "9", "8", "6", "2", "4", "1"],
            ["2", "6", "4", "3", "1", "7", "5", "9", "8"],
            ["1", "9", "8", "5", "2", "4", "3", "6", "7"],
            ["9", "7", "5", "8", "6", "3", "1", "2", "4"],
            ["8", "3", "2", "4", "9", "1", "7", "5", "6"],
            ["6", "4", "1", "2", "7", "5", "9", "8", "3"],
        ]
        .iter()
        .map(|l| l.iter().map(|l| l.chars().next().unwrap()).collect())
        .collect::<Vec<_>>();

        solve_sudoku(&mut board);

        assert_eq!(board, solution);
    }

    #[test]
    fn test_3() {
        let mut board = [
            [".", ".", ".", "2", ".", ".", ".", "6", "3"],
            ["3", ".", ".", ".", ".", "5", "4", ".", "1"],
            [".", ".", "1", ".", ".", "3", "9", "8", "."],
            [".", ".", ".", ".", ".", ".", ".", "9", "."],
            [".", ".", ".", "5", "3", "8", ".", ".", "."],
            [".", "3", ".", ".", ".", ".", ".", ".", "."],
            [".", "2", "6", "3", ".", ".", "5", ".", "."],
            ["5", ".", "3", "7", ".", ".", ".", ".", "8"],
            ["4", "7", ".", ".", ".", "1", ".", ".", "."],
        ]
        .iter()
        .map(|l| l.iter().map(|l| l.chars().next().unwrap()).collect())
        .collect::<Vec<_>>();

        let solution: Vec<Vec<char>> = [
            ["8", "5", "4", "2", "1", "9", "7", "6", "3"],
            ["3", "9", "7", "8", "6", "5", "4", "2", "1"],
            ["2", "6", "1", "4", "7", "3", "9", "8", "5"],
            ["7", "8", "5", "1", "2", "6", "3", "9", "4"],
            ["6", "4", "9", "5", "3", "8", "1", "7", "2"],
            ["1", "3", "2", "9", "4", "7", "8", "5", "6"],
            ["9", "2", "6", "3", "8", "4", "5", "1", "7"],
            ["5", "1", "3", "7", "9", "2", "6", "4", "8"],
            ["4", "7", "8", "6", "5", "1", "2", "3", "9"],
        ]
        .iter()
        .map(|l| l.iter().map(|l| l.chars().next().unwrap()).collect())
        .collect::<Vec<_>>();

        solve_sudoku(&mut board);

        assert_eq!(board, solution);
    }
}
