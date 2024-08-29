use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn word_pattern(pattern: String, s: String) -> bool {
        word_pattern(&pattern, &s)
    }
}

fn word_pattern(pattern: &str, s: &str) -> bool {
    let mut hash_map_s_to_c = HashMap::new();
    let mut hash_map_p_to_s = HashMap::new();

    let mut pattern_split = pattern.chars();
    let mut string_split = s.split(' ');

    loop {
        let (p, s) = match (pattern_split.next(), string_split.next()) {
            (None, None) => break,
            (None, Some(_)) | (Some(_), None) => return false,
            (Some(p), Some(s)) => (p, s),
        };

        match (hash_map_s_to_c.entry(p), hash_map_p_to_s.entry(s)) {
            (Entry::Occupied(o1), Entry::Occupied(o2)) => {
                if o1.get() != &s || o2.get() != &p {
                    return false;
                }
            },
            (Entry::Occupied(_), Entry::Vacant(_)) | (Entry::Vacant(_), Entry::Occupied(_)) => {
                return false
            },
            (Entry::Vacant(v1), Entry::Vacant(v2)) => {
                v1.insert(s);
                v2.insert(p);
            },
        }
    }

    true
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0290::word_pattern;

    #[test]
    fn test_1() {
        let pattern = "abba";
        let s = "dog cat cat dog";
        assert!(word_pattern(pattern, s));
    }

    #[test]
    fn test_2() {
        let pattern = "abba";
        let s = "dog cat cat fish";
        assert!(!word_pattern(pattern, s));
    }

    #[test]
    fn test_3() {
        let pattern = "aaaa";
        let s = "dog cat cat dog";

        assert!(!word_pattern(pattern, s));
    }
}
