use crate::shared::Solution;

fn word_break(s: &str, word_dict: &[&str]) -> bool {
    if s.is_empty() {
        return true;
    }

    for i in (1..=s.len()).rev() {
        let (left, right) = s.split_at(i);

        if word_dict.contains(&left) && word_break(right, word_dict) {
            return true;
        }
    }

    false
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let strs = word_dict.iter().map(String::as_str).collect::<Vec<_>>();

        word_break(&s, &strs)
    }
}

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
}
