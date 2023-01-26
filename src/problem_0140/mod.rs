use std::collections::HashMap;

fn word_break_r<'a>(
    cache: &mut HashMap<&'a str, Vec<String>>,
    s: &'a str,
    word_dict: &[&str],
) -> Vec<String> {
    if let Some(val) = cache.get(s) {
        return val.clone();
    }

    let mut result = Vec::new();

    for word in word_dict.iter() {
        if let Some(stripped) = s.strip_prefix(word) {
            for outer_word in word_break_r(cache, stripped, word_dict) {
                result.push([word, " ", outer_word.as_str()].concat());
            }

            if &s == word {
                result.push((*word).to_string());
            }
        }
    }

    cache.insert(s, result.clone());
    result
}

fn word_break(s: &str, word_dict: &[&str]) -> Vec<String> {
    word_break_r(&mut HashMap::new(), s, word_dict)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let strs = word_dict.iter().map(String::as_str).collect::<Vec<_>>();

        word_break(&s, &strs)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0140::word_break;

    #[test]
    fn test_1() {
        assert_eq!(
            word_break("catsanddog", &["cat", "cats", "and", "sand", "dog"]),
            &["cat sand dog", "cats and dog",]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            word_break(
                "pineapplepenapple",
                &["apple", "pen", "applepen", "pine", "pineapple"]
            ),
            &[
                "pine apple pen apple",
                "pine applepen apple",
                "pineapple pen apple",
            ]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            word_break("catsandog", &["cats", "dog", "sand", "and", "cat"]),
            &[] as &[String]
        );
    }
}
