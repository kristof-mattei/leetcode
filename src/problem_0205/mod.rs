use std::collections::{HashMap, HashSet};

fn is_isomorphic<T: AsRef<str>>(s: T, t: T) -> bool {
    if s.as_ref().len() != t.as_ref().len() {
        return false;
    }

    let mut map = HashMap::<u8, u8>::new();
    let mut seen_right = HashSet::<u8>::new();

    for (l, r) in s
        .as_ref()
        .as_bytes()
        .iter()
        .zip(t.as_ref().as_bytes().iter())
    {
        if let Some(expected) = map.get(l) {
            if expected != r {
                return false;
            }
        } else if seen_right.contains(r) {
            return false;
        } else {
            seen_right.insert(*r);
            map.insert(*l, *r);
        }
    }

    true
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_isomorphic(s: String, t: String) -> bool {
        is_isomorphic(s, t)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0205::is_isomorphic;

    #[test]
    fn test_1() {
        assert!(is_isomorphic("egg", "add"));
    }

    #[test]
    fn test_2() {
        assert!(!is_isomorphic("foo", "bar"));
    }

    #[test]
    fn test_3() {
        assert!(is_isomorphic("paper", "title"));
    }

    #[test]
    fn test_4() {
        assert!(!is_isomorphic("badc", "baba"));
    }
}
