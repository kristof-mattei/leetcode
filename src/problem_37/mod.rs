use crate::shared::Solution;

fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
    if is_solved(board) {
        return true;
    }

    let (i, j, mask) = get_cell_with_lowest_p(board);

    for c in mask
        .iter()
        .enumerate()
        .filter(|(_, x)| **x)
        .map(|(i, _)| (i as u8 + b'0') as char)
    {
        board[i][j] = c;

        if solve_sudoku(board) {
            return true;
        }

        board[i][j] = '.';
    }

    false
}

fn is_solved(board: &[Vec<char>]) -> bool {
    for r in board {
        for c in r {
            if c == &'.' {
                return false;
            }
        }
    }

    true
}

fn get_cell_with_lowest_p(board: &[Vec<char>]) -> (usize, usize, [bool; 10]) {
    // first element is ignored
    // efficient storage for possibilities fora cell
    // if mask[i][j][c] is set, that means that c + 1 as char is a possibility
    let mut masks = [[[false, true, true, true, true, true, true, true, true, true]; 9]; 9];

    for i in 0..=8 {
        for j in 0..=8 {
            let c = board[i][j];

            if c == '.' {
                continue;
            }

            let c = (c as u8 - b'0') as usize;

            // unset all the fields that have our current number
            for t in 0..=8 {
                // for this row (i), for all columns (t), set that c isn't a possibility
                masks[i][t][c] = false;
                // for all rows (t), for this column (c), set that c isn't a possibility
                masks[t][j][c] = false;
                // for our current quadrant, set that c isn't a possibility
                masks[i / 3 * 3 + t / 3][j / 3 * 3 + t % 3][c] = false;
            }
        }
    }

    let mut min = (10, 0, 0);

    for i in 0..=8 {
        for j in 0..=8 {
            if board[i][j] != '.' {
                continue;
            }

            min = min.min((masks[i][j].iter().filter(|x| **x).count(), i, j));
        }
    }

    (min.1, min.2, masks[min.1][min.2])
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        solve_sudoku(board);
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_37::solve_sudoku;

    #[test]
    fn test_1() {
        let mut board = [
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
