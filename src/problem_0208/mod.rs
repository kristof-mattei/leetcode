#![allow(dead_code)]
use std::iter::Peekable;

#[allow(dead_code)]
#[derive(Default, Debug)]
struct Trie {
    states: Vec<State>,
}

#[derive(Default, Debug)]
struct State([u16; 26], bool);

impl State {
    pub fn next(&self, letter: u8) -> Option<usize> {
        let lookup_index = (letter - b'a') as usize;

        let next_index = self.0[lookup_index];

        (next_index != 0).then(|| next_index as usize)
    }

    pub fn is_complete(&self) -> bool {
        self.1
    }

    pub fn next_or_insert(&mut self, letter: u8, default_index: usize) -> usize {
        if let Some(next) = self.next(letter) {
            next
        } else {
            let lookup_index = (letter - b'a') as usize;
            self.0[lookup_index] = default_index as u16;
            default_index
        }
    }

    pub fn set_complete(&mut self) {
        self.1 = true;
    }
}

impl Trie {
    fn new() -> Self {
        Self {
            states: vec![State::default()],
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn insert(&mut self, word: String) {
        println!("Inserting {word}");
        let mut iter = word.as_bytes().iter().copied().peekable();

        let mut index = self.follow(&mut iter);

        println!("Starting index: {index}");

        for letter in iter {
            let new_index = self.states.len();
            println!("New index: {new_index}");
            let next_index = self.states[index].next_or_insert(letter, new_index);
            println!("Next index: {next_index}");
            if next_index == new_index {
                self.states.push(State::default());
            }
            index = next_index;
        }

        self.states[index].set_complete();
    }

    #[allow(clippy::needless_pass_by_value)]
    fn search(&self, word: String) -> bool {
        let mut iter = word.as_bytes().iter().copied().peekable();

        let index = self.follow(&mut iter);
        iter.next().is_none() && self.states[index].is_complete()
    }

    #[allow(clippy::needless_pass_by_value)]
    fn starts_with(&self, prefix: String) -> bool {
        let mut iter = prefix.as_bytes().iter().copied().peekable();
        self.follow(&mut iter);
        iter.next().is_none()
    }

    fn follow(&self, iter: &mut Peekable<impl Iterator<Item = u8>>) -> usize {
        let mut index = 0;

        while let Some(next) = iter.peek().and_then(|&c| self.states[index].next(c)) {
            iter.next();
            index = next;
        }

        index
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0208::Trie;

    #[test]
    fn test_1() {
        let mut trie = Trie::new();

        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));

        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }

    #[test]
    fn test_2() {
        let mut trie = Trie::new();

        trie.insert("abcd".to_string());
        trie.insert("aac".to_string());
        assert!(!trie.search("abc".to_string()));
    }
}
