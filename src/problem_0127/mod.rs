use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        ladder_length(&begin_word, &end_word, &word_list)
    }
}

fn ladder_length(begin_word: &str, end_word: &str, word_list: &[String]) -> i32 {
    let cache = build_cache(word_list, begin_word);

    let mut word_queue = VecDeque::from_iter([(begin_word, 1)]);

    let mut history = HashSet::new();

    while let Some((from, length)) = word_queue.pop_front() {
        if from == end_word {
            return length;
        }

        if !history.insert(from) {
            continue;
        }

        for to in word_list {
            if cache.get(from).is_some_and(|t| t.contains(to)) {
                word_queue.push_back((to, length + 1));
            }
        }
    }

    0
}

fn can_move(from_word: &str, to_word: &str) -> bool {
    let mut can_move = false;

    for (f, t) in from_word.as_bytes().iter().zip(to_word.as_bytes()) {
        if f != t {
            if can_move {
                return false;
            }

            can_move = true;
        }
    }

    can_move
}

fn build_cache(word_list: &[String], begin_word: &str) -> HashMap<String, HashSet<String>> {
    let mut cache = HashMap::<String, HashSet<String>>::new();

    for from_word in word_list.iter().chain([&String::from(begin_word)]) {
        let mut targets = HashSet::<String>::new();

        for to_word in word_list {
            if can_move(from_word, to_word) {
                targets.insert(to_word.clone());
            }
        }

        cache.insert(from_word.clone(), targets);
    }

    cache
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0127::ladder_length;

    #[test]
    fn test_1() {
        assert_eq!(
            ladder_length(
                "hit",
                "cog",
                &["hot", "dot", "dog", "lot", "log", "cog"]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
            ),
            5
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            ladder_length(
                "hit",
                "cog",
                &["hot", "dot", "dog", "lot", "log"]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
            ),
            0
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            ladder_length(
                "hot",
                "dog",
                &["hot", "cog", "dog", "tot", "hog", "hop", "pot", "dot"]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
            ),
            3
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            ladder_length(
                "qa",
                "sq",
                &[
                    "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm",
                    "le", "av", "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo",
                    "ow", "sn", "ya", "cr", "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur",
                    "rh", "sr", "tc", "lt", "lo", "as", "fr", "nb", "yb", "if", "pb", "ge", "th",
                    "pm", "rb", "sh", "co", "ga", "li", "ha", "hz", "no", "bi", "di", "hi", "qa",
                    "pi", "os", "uh", "wm", "an", "me", "mo", "na", "la", "st", "er", "sc", "ne",
                    "mn", "mi", "am", "ex", "pt", "io", "be", "fm", "ta", "tb", "ni", "mr", "pa",
                    "he", "lr", "sq", "ye"
                ]
                .into_iter()
                .map(Into::into)
                .collect::<Vec<String>>()
            ),
            5
        );
    }
}
