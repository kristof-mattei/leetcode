#![allow(dead_code)]

#[derive(Default, Debug)]
struct Trie {
    states: Vec<State>,
}

#[derive(Default, Debug)]
struct State([u16; 26], bool);

impl State {
    pub fn next(&self, letter: u8) -> Option<usize> {
        let lookup_index = (letter - b'a') as usize;

        // next index is the index in `Trie.states[]`
        let next_index = self.0[lookup_index];

        // a next_index of `0` means there's no state in `Trie.states` following the current state
        // as `Trie.states[0]` is actually the entrypoint state, and is never used outside of that
        // TODO: self.0 should be of type [Option<u16>;26] so that we don't need to rely
        // on 0 being a magic number
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
        let mut last_matching_index = self.follow(&word);

        for letter in word.bytes().into_iter().skip(last_matching_index) {
            let new_index = self.states.len();

            let next_index = self.states[last_matching_index].next_or_insert(letter, new_index);

            if next_index == new_index {
                self.states.push(State::default());
            }

            last_matching_index = next_index;
        }

        self.states[last_matching_index].set_complete();
    }

    #[allow(clippy::needless_pass_by_value)]
    fn search(&self, word: String) -> bool {
        let index = self.follow(&word);
        word.as_bytes().get(index + 1).is_none() && self.states[index].is_complete()
    }

    #[allow(clippy::needless_pass_by_value)]
    fn starts_with(&self, prefix: String) -> bool {
        let index = self.follow(&prefix);
        prefix.as_bytes().get(index + 1).is_none()
    }

    /// Follows the bytes in `word` against `self.states`
    /// and returns the last matching index
    fn follow(&self, word: &str) -> usize {
        word.as_bytes()
            .iter()
            .try_fold(0, |acc, curr| self.states[acc].next(*curr).ok_or(acc))
            .map_or_else(|e| e, |o| o)
    }
}

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
