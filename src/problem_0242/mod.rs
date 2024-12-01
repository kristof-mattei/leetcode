impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value)]
    pub fn is_anagram(s: String, t: String) -> bool {
        is_anagram(&s, &t)
    }
}

fn is_anagram(s: &str, t: &str) -> bool {
    const A: usize = 'a' as usize;

    if s.len() != t.len() {
        return false;
    }

    let mut frequencies_s = s.chars().fold(vec![0usize; 26], |mut acc, c| {
        acc[c as usize - A] += 1;
        acc
    });

    for c in t.chars() {
        match frequencies_s.get_mut(c as usize - A) {
            None | Some(0) => return false,
            Some(c) => {
                *c -= 1;
            },
        }
    }

    true
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0242::is_anagram;

    #[test]
    fn test_1() {
        assert!(is_anagram("anagram", "nagaram"));
    }

    #[test]
    fn test_2() {
        assert!(!is_anagram("rat", "car"));
    }
}
