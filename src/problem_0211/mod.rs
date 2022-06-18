#![allow(dead_code)]
struct WordDictionary {}

impl WordDictionary {
    fn new() -> Self {
        Self {}
    }

    // #[allow(clippy::needless_pass_by_value)]
    // fn add_word(&mut self, _word: String) {
    //     unimplemented!()
    // }

    // #[allow(clippy::needless_pass_by_value)]
    // fn search(&self, _word: String) -> bool {
    //     unimplemented!()
    // }
}

#[cfg(test)]
mod tests {
    // use crate::problem_0211::WordDictionary;

    #[test]
    fn test_1() {
        // let mut word_dictionary = WordDictionary::new();

        // word_dictionary.add_word("bad".to_string());
        // word_dictionary.add_word("dad".to_string());
        // word_dictionary.add_word("mad".to_string());

        // assert!(!word_dictionary.search("pad".to_string())); // return False
        // assert!(word_dictionary.search("bad".to_string())); // return True
        // assert!(word_dictionary.search(".ad".to_string())); // return True
        // assert!(word_dictionary.search("b..".to_string())); // return True
    }
}
