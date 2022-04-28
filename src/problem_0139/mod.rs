use std::collections::HashMap;

fn word_break_r<'a>(
    cache: &mut HashMap<&'a str, bool>,
    s: &'a str,
    max_len: usize,
    word_dict: &[&str],
) -> bool {
    if let Some(&c) = cache.get(s) {
        return c;
    }

    if s.is_empty() {
        return true;
    }

    for i in (1..=usize::min(max_len, s.len())).rev() {
        let (left, right) = s.split_at(i);

        if word_dict.contains(&left) && word_break_r(cache, right, max_len, word_dict) {
            cache.insert(s, true);
            return true;
        }
    }

    cache.insert(s, false);
    false
}

fn word_break(s: &str, word_dict: &[&str]) -> bool {
    let mut letters_in_dict = [false; 26];

    for w in word_dict {
        for c in w.as_bytes() {
            letters_in_dict[(c - b'a') as usize] = true;
        }
    }

    for c in s.as_bytes() {
        if !letters_in_dict[(c - b'a') as usize] {
            return false;
        }
    }

    let max = unsafe { word_dict.iter().map(|s| s.len()).max().unwrap_unchecked() };

    word_break_r(&mut HashMap::new(), s, max, word_dict)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let strs = word_dict.iter().map(String::as_str).collect::<Vec<_>>();

        word_break(&s, &strs)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0139::word_break;

    #[test]
    fn test_1() {
        assert!(word_break("leetcode", &["leet", "code"]));
    }

    #[test]
    fn test_2() {
        assert!(word_break("applepenapple", &["apple", "pen"]));
    }

    #[test]
    fn test_3() {
        assert!(!word_break(
            "catsandog",
            &["cats", "dog", "sand", "and", "cat"]
        ));
    }

    #[test]
    fn test_4() {
        assert!(!word_break(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab",
            &[
                "a",
                "aa",
                "aaa",
                "aaaa",
                "aaaaa",
                "aaaaaa",
                "aaaaaaa",
                "aaaaaaaa",
                "aaaaaaaaa",
                "aaaaaaaaaa"
            ]
        ));
    }

    #[test]
    fn test_5() {
        assert!(word_break("foo", &["foo"]));
    }
}
