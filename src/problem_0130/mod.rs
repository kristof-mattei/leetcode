fn mark_touching_o_s(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
    if board[r][c] == 'X' {
        return;
    }

    if board[r][c] == 'O' {
        return;
    }

    // it's not an X and we only come here from a side that IS an O
    board[r][c] = 'O';

    if r != 0 {
        mark_touching_o_s(board, r - 1, c);
    }
    if r != board.len() - 1 {
        mark_touching_o_s(board, r + 1, c);
    }
    if c != 0 {
        mark_touching_o_s(board, r, c - 1);
    }
    if c != board[0].len() - 1 {
        mark_touching_o_s(board, r, c + 1);
    }
}

fn solve(board: &mut Vec<Vec<char>>) {
    // we're going to switch all Os to Zs
    // then for each Z on the edge we switch those to Os until we've completed the
    // traversal

    // lastly we mark all the remaining Zs as X, as we didn't hit them from an edge, ergo they are not touching an edge

    for row in board.iter_mut() {
        for col in row.iter_mut() {
            if *col == 'O' {
                *col = 'Z';
            }
        }
    }

    let rows = board.len();
    let cols = board[0].len();

    let coordinates_to_start = std::iter::empty()
        .chain((0..rows).map(|r| (r, 0)))
        .chain((0..rows).map(|r| (r, cols - 1)))
        .chain((1..cols - 1).map(|c| (0, c)))
        .chain((1..cols - 1).map(|c| (rows - 1, c)));

    for (r, c) in coordinates_to_start {
        mark_touching_o_s(board, r, c);
    }

    for row in board.iter_mut() {
        for col in row.iter_mut() {
            if *col == 'Z' {
                *col = 'X';
            }
        }
    }
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        solve(board);
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0130::solve;

    #[test]
    fn test_1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        let expected = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];

        solve(&mut board);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_2() {
        let mut board = vec![vec!['X']];
        let expected = vec![vec!['X']];

        solve(&mut board);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_3() {
        let mut board = vec![
            vec!['O', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'X', 'O'],
            vec!['X', 'O', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'O', 'O'],
            vec!['X', 'X', 'O', 'X', 'O'],
        ];

        let expected = vec![
            ['O', 'X', 'X', 'O', 'X'],
            ['X', 'X', 'X', 'X', 'O'],
            ['X', 'X', 'X', 'O', 'X'],
            ['O', 'X', 'O', 'O', 'O'],
            ['X', 'X', 'O', 'X', 'O'],
        ];

        solve(&mut board);
        assert_eq!(board, expected);
    }
}
