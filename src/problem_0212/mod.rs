/// used to turn a char into a 0-based index
const LETTER_OFFSET: u8 = b'a';
/// letters in the alphabet
const NUM_LETTERS: usize = 26;
/// replacement character to define processing of current i,j
const PROCESSING_PLACEHOLDER: char = '*';

#[derive(Default)]
struct Trie {
    word: Option<String>,
    word_count: u32,
    children: [Option<Box<Trie>>; NUM_LETTERS],
}

fn check_cell(
    row: usize,
    col: usize,
    m: usize,
    n: usize,
    board: &mut Vec<Vec<char>>,
    trie: &mut Trie,
    result: &mut Vec<String>,
) -> u32 {
    if board[row][col] == PROCESSING_PLACEHOLDER {
        return 0;
    }

    let mut found_words = 0;

    let index = (board[row][col] as u8 - LETTER_OFFSET) as usize;

    if let Some(mut child) = trie.children[index].take() {
        if let Some(word) = child.word.take() {
            result.push(word);
            found_words += 1;
        }

        let character = board[row][col];

        board[row][col] = PROCESSING_PLACEHOLDER;

        if row > 0 {
            found_words += check_cell(row - 1, col, m, n, board, &mut child, result);
        }
        if col > 0 {
            found_words += check_cell(row, col - 1, m, n, board, &mut child, result);
        }
        if row + 1 < m {
            found_words += check_cell(row + 1, col, m, n, board, &mut child, result);
        }
        if col + 1 < n {
            found_words += check_cell(row, col + 1, m, n, board, &mut child, result);
        }

        board[row][col] = character;

        if found_words < child.word_count {
            trie.children[index] = Some(child);
        }
    }

    found_words
}

fn find_words(mut board: Vec<Vec<char>>, mut words: Vec<String>) -> Vec<String> {
    let mut trie = Trie::default();

    for word in words.drain(..) {
        let mut node = &mut trie;
        for &b in word.as_bytes() {
            node.word_count += 1;
            node = node.children[(b - LETTER_OFFSET) as usize].get_or_insert_with(Default::default);
        }
        node.word_count += 1;
        node.word = Some(word);
    }

    // check board
    let m = board.len();
    let n = board[0].len();

    for row in 0..m {
        for col in 0..n {
            check_cell(row, col, m, n, &mut board, &mut trie, &mut words);
        }
    }

    words
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        find_words(board, words)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0212::find_words;
    use crate::svec;

    #[test]
    fn test_1() {
        let mut result = find_words(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            svec!["oath", "pea", "eat", "rain"],
        );

        result.sort();

        assert_eq!(result, ["eat", "oath"]);
    }

    #[test]
    fn test_2() {
        let mut result = find_words(vec![vec!['a']], svec!["a"]);

        result.sort();

        assert_eq!(result, ["a"]);
    }
}
