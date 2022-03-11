use crate::shared::Solution;

fn exist_r(board: &mut [Vec<char>], word: &[char], row: usize, col: usize) -> bool {
    if word.is_empty() {
        return true;
    }

    board[row][col] = (board[row][col] as u8 ^ 0xff) as char;

    let letter_match = ({
        row != 0 && board[row - 1][col] == word[0] && exist_r(board, &word[1..], row - 1, col)
    }) || ({
        col != 0 && board[row][col - 1] == word[0] && exist_r(board, &word[1..], row, col - 1)
    }) || ({
        row != board.len() - 1
            && board[row + 1][col] == word[0]
            && exist_r(board, &word[1..], row + 1, col)
    }) || ({
        col != board[0].len() - 1
            && board[row][col + 1] == word[0]
            && exist_r(board, &word[1..], row, col + 1)
    });

    board[row][col] = (board[row][col] as u8 ^ 0xff) as char;
    letter_match
}

fn exist(mut board: Vec<Vec<char>>, word: &str) -> bool {
    let chars = word.chars().collect::<Vec<_>>();

    let rows = board.len();
    let cols = board[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if board[r][c] == chars[0] && exist_r(&mut board, &chars[1..], r, c) {
                return true;
            }
        }
    }

    false
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        exist(board, &word)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0079::exist;

    #[test]
    fn test_1() {
        assert!(exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED"
        ));
    }

    #[test]
    fn test_2() {
        assert!(exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE"
        ));
    }

    #[test]
    fn test_3() {
        assert!(!exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB"
        ));
    }
}
