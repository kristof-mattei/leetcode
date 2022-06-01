use std::iter::Peekable;

#[allow(dead_code)]
#[derive(Default)]
struct Trie {
    states: Vec<State>,
}

#[derive(Default)]
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

    fn insert(&mut self, word: String) {
        let mut iter = word.as_bytes().iter().copied().peekable();
        let mut index = self.follow(&mut iter);
        for letter in iter {
            let new_index = self.states.len();
            let next_index = self.states[index].next_or_insert(letter, new_index);
            if next_index == new_index {
                self.states.push(State::default());
            }
            index = next_index;
        }

        self.states[index].set_complete();
    }

    fn search(&self, word: String) -> bool {
        let mut iter = word.as_bytes().iter().copied().peekable();
        let index = self.follow(&mut iter);
        iter.next().is_none() && self.states[index].is_complete()
    }

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
        assert!(trie.search("apple".to_string())); // return True
        assert!(!trie.search("app".to_string())); // return False
        assert!(trie.starts_with("app".to_string())); // return True
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string())); // return True
    }
}
