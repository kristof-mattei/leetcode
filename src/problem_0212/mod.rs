use std::collections::HashSet;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
    index: Option<usize>,
}

fn insert(mut trie: &mut Box<Trie>, word: &[u8], index: usize) {
    for b in word {
        let child_index = (b - b'a') as usize;
        trie = trie.children[child_index].get_or_insert(Box::new(Trie::default()));
    }
    trie.is_end = true;
    trie.index = Some(index);
}

fn search(
    i: usize,
    j: usize,
    n: usize,
    m: usize,
    board: &mut Vec<Vec<char>>,
    answers: &mut HashSet<usize>,
    root: &Trie,
) {
    if (board[i][j] as u8 >> 7 & 1u8 == 1u8)
        || root.children[((board[i][j] as u8 - b'a') as usize)].is_none()
    {
        return;
    }

    let root = root.children[((board[i][j] as u8 - b'a') as usize)]
        .as_ref()
        .unwrap();

    if root.is_end {
        answers.insert(root.index.unwrap());
    }

    board[i][j] = (board[i][j] as u8 | (1u8 << 7)) as char;

    if i + 1 < n {
        search(i + 1, j, n, m, board, answers, root);
    }
    if j + 1 < m {
        search(i, j + 1, n, m, board, answers, root);
    }

    if j > 0 {
        search(i, j - 1, n, m, board, answers, root);
    }
    if i > 0 {
        search(i - 1, j, n, m, board, answers, root);
    }

    board[i][j] = (board[i][j] as u8 & !(1u8 << 7)) as char;
}

fn find_words(mut board: Vec<Vec<char>>, words: &[String]) -> Vec<String> {
    let rows = board.len();
    let columns = board[0].len();

    let mut root = Box::new(Trie::default());

    for (i, word) in words.iter().enumerate() {
        insert(&mut root, word.as_bytes(), i);
    }

    let mut answer_indeces = HashSet::new();

    for row_index in 0..rows {
        for col_index in 0..columns {
            search(
                row_index,
                col_index,
                rows,
                columns,
                &mut board,
                &mut answer_indeces,
                &root,
            );
        }
    }

    let mut result = vec![];

    for i in answer_indeces {
        result.push(words[i].clone());
    }
    result
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        find_words(board, &words)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0212::find_words, svec};

    #[test]
    fn test_1() {
        let mut result = find_words(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            &svec!["oath", "pea", "eat", "rain"],
        );

        result.sort();

        assert_eq!(result, ["eat", "oath"]);
    }

    #[test]
    fn test_2() {
        let mut result = find_words(vec![vec!['a']], &svec!["a"]);

        result.sort();

        assert_eq!(result, ["a"]);
    }

    #[test]
    fn test_3() {
        for i in 'a'..='z' {
            // println!("{},{:#010b},{:#010b}", i, i as u8, (i as u8 ^ 0xff));
            println!(
                "{},{:#010b},{:#010b},{:#010b}",
                i,
                i as u8,
                (i as u8) | 0b1000_0000,
                ((i as u8) & !(1u8 << 7))
            );
        }
    }
}
