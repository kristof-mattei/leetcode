fn process(c: char) -> Option<char> {
    match c.to_ascii_lowercase() {
        l @ ('0'..='9' | 'a'..='z') => Some(l),
        _ => None,
    }
}

fn is_palindrome(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();

    let mut start = 0;

    let mut end = chars.len() - 1;

    while start < end {
        let left = process(chars[start]);

        if left.is_none() {
            start += 1;
            continue;
        }

        let right = process(chars[end]);

        if right.is_none() {
            end -= 1;
            continue;
        }

        if left.unwrap() == right.unwrap() {
            start += 1;
            end -= 1;
            continue;
        }

        return false;
    }

    true
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_palindrome_s(s: String) -> bool {
        is_palindrome(&s)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0125::is_palindrome;

    #[test]
    fn test_1() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
    }

    #[test]
    fn test_2() {
        assert!(!is_palindrome("race a car"));
    }

    #[test]
    fn test_3() {
        assert!(is_palindrome(" "));
    }
}
