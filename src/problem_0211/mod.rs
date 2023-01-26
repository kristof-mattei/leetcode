#![allow(dead_code)]
#[derive(Default, Debug)]
struct WordDictionary {
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
        (next_index != 0).then_some(next_index as usize)
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

impl WordDictionary {
    fn new() -> Self {
        Self {
            states: vec![State::default()],
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn add_word(&mut self, word: String) {
        let (to_skip, mut last_matching_index) = self.follow(&word);

        for letter in word.bytes().skip(to_skip) {
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
    fn search_r(&self, word: &[u8], mut acc: usize) -> bool {
        for (index, &letter) in word.iter().enumerate() {
            if letter == b'.' {
                for s in self.states[acc]
                    .0
                    .iter()
                    .filter_map(|&o| (o != 0).then_some(o as usize))
                {
                    if self.search_r(&word[(index + 1)..], s) {
                        return true;
                    }
                }

                return false;
            } else if let Some(new_acc) = self.states[acc].next(letter) {
                acc = new_acc;
            } else {
                return false;
            }
        }

        self.states[acc].is_complete()
    }

    #[allow(clippy::needless_pass_by_value)]
    fn search(&self, word: String) -> bool {
        self.search_r(word.as_bytes(), 0)
    }

    /// Follows the bytes in `word` against `self.states`
    /// and returns the last matching index
    fn follow(&self, word: &str) -> (usize, usize) {
        let mut acc = 0;

        for (index, letter) in word.as_bytes().iter().enumerate() {
            if let Some(new_acc) = self.states[acc].next(*letter) {
                acc = new_acc;
            } else {
                return (index, acc);
            }
        }

        (word.len(), acc)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0211::WordDictionary;

    #[test]
    fn test_1() {
        let mut word_dictionary = WordDictionary::new();

        word_dictionary.add_word("bad".to_string());
        word_dictionary.add_word("dad".to_string());
        word_dictionary.add_word("mad".to_string());

        assert!(!word_dictionary.search("pad".to_string())); // return False
        assert!(word_dictionary.search("bad".to_string())); // return True
        assert!(word_dictionary.search(".ad".to_string())); // return True
        assert!(word_dictionary.search("b..".to_string())); // return True
    }

    #[test]
    fn test_2() {
        let mut word_dictionary = WordDictionary::new();

        word_dictionary.add_word("zxxxx".to_string());
        word_dictionary.add_word("qeba".to_string());
        word_dictionary.add_word("qes".to_string());

        assert!(word_dictionary.search("...".to_string()));
    }

    #[test]
    fn test_3() {
        let mut word_dictionary = WordDictionary::new();

        word_dictionary.add_word("jbtjyj".to_string());
        word_dictionary.add_word("qsibzxao".to_string());
        word_dictionary.add_word("qes".to_string());

        assert!(word_dictionary.search("qes".to_string()));
    }
    #[test]
    fn test_4() {
        let mut word_dictionary = WordDictionary::new();

        word_dictionary.add_word("a".to_string());
        word_dictionary.add_word("a".to_string());

        assert!(word_dictionary.search(".".to_string()));
        assert!(word_dictionary.search("a".to_string()));
        assert!(!word_dictionary.search("aa".to_string()));
        assert!(word_dictionary.search("a".to_string()));
        assert!(!word_dictionary.search(".a".to_string()));
        assert!(!word_dictionary.search("a.".to_string()));
    }
}
