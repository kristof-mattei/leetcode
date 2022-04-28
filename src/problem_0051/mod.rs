fn can_place(board: &[(usize, usize)], n: usize, row: usize, col: usize) -> bool {
    // is there a queen where we are
    // if board[row][col] {
    //     return false;
    // }

    // this one is not needed as we shift row by row, we never write to the same row
    // any on current row
    // let on_current_row = board.iter().any(|&(r, _)| r == row);

    // if on_current_row {
    //     return false;
    // }

    // any on current col
    let on_current_col = board.iter().any(|&(_, c)| c == col);

    if on_current_col {
        return false;
    }
    // current location to left top
    for i in 1..=usize::min(row, col) {
        if board.contains(&(row - i, col - i)) {
            return false;
        }
    }

    // left bottom
    for i in 1..=usize::min(n - row - 1, col) {
        if board.contains(&(row + i, col - i)) {
            return false;
        }
    }

    // right top
    for i in 1..=usize::min(row, n - col - 1) {
        if board.contains(&(row - i, col + i)) {
            return false;
        }
    }

    // right bottom
    for i in 1..=usize::min(n - row - 1, n - col - 1) {
        if board.contains(&(row + i, col + i)) {
            return false;
        }
    }

    true
}

fn solve_n_queens_r(
    board: &mut Vec<(usize, usize)>,
    n: usize,
    remaining: usize,
    i_start: usize,
) -> Vec<Vec<(usize, usize)>> {
    if remaining == 0 {
        return vec![board.clone()];
    }

    let mut results = Vec::new();

    for j in 0..n {
        if can_place(board, n, i_start, j) {
            board.push((i_start, j));

            let r = solve_n_queens_r(board, n, remaining - 1, i_start + 1);

            for b in r {
                let found = b.len();

                if found < remaining {
                    continue;
                }

                results.push(b);
            }

            board.pop();
        }
    }

    results
}

pub(crate) fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    if n == 2 || n == 3 {
        return vec![];
    }

    solve_n_queens_r(&mut Vec::new(), n as usize, n as usize, 0)
        .into_iter()
        .map(|b| {
            let mut board = vec![vec!['.'; n as usize]; n as usize];

            for q in b {
                board[q.0][q.1] = 'Q';
            }

            let r = board
                .into_iter()
                .map(|r| r.into_iter().collect::<String>())
                .collect::<Vec<_>>();

            r
        })
        .collect()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        solve_n_queens(n)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use crate::{problem_0051::solve_n_queens, shared::vec_eq};

    #[test]
    fn test_1() {
        let expected = vec![
            vec![".Q..", "...Q", "Q...", "..Q."],
            vec!["..Q.", "Q...", "...Q", ".Q.."],
        ]
        .into_iter()
        .map(|v| v.into_iter().map(ToString::to_string).collect())
        .collect();

        let result = solve_n_queens(4);
        assert!(vec_eq(result, expected));
    }

    #[test]
    fn test_2() {
        let expected = vec![vec!["Q"]];

        assert_eq!(solve_n_queens(1), expected);
    }

    #[test]
    fn test_3() {
        let expected = vec![
            vec!["Q....", "..Q..", "....Q", ".Q...", "...Q."],
            vec!["Q....", "...Q.", ".Q...", "....Q", "..Q.."],
            vec![".Q...", "...Q.", "Q....", "..Q..", "....Q"],
            vec![".Q...", "....Q", "..Q..", "Q....", "...Q."],
            vec!["..Q..", "Q....", "...Q.", ".Q...", "....Q"],
            vec!["..Q..", "....Q", ".Q...", "...Q.", "Q...."],
            vec!["...Q.", "Q....", "..Q..", "....Q", ".Q..."],
            vec!["...Q.", ".Q...", "....Q", "..Q..", "Q...."],
            vec!["....Q", ".Q...", "...Q.", "Q....", "..Q.."],
            vec!["....Q", "..Q..", "Q....", "...Q.", ".Q..."],
        ]
        .into_iter()
        .map(|v| v.into_iter().map(ToString::to_string).collect())
        .collect();

        let result = solve_n_queens(5);
        assert!(vec_eq(result, expected));
    }
}
