use std::collections::HashSet;

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn remove_invalid_parentheses(input: String) -> Vec<String> {
        remove_invalid_parentheses(&input)
            .into_iter()
            .collect::<Vec<String>>()
    }
}

fn remove_invalid_parentheses(input: &str) -> HashSet<String> {
    let s = input.chars().collect::<Vec<_>>();

    if is_valid(&s) {
        return [input.to_owned()].into_iter().collect();
    }

    remove_invalid_parentheses_r([s].into_iter().collect::<HashSet<_>>())
        .iter()
        .map(|s| s.iter().collect::<String>())
        .collect()
}

fn remove_invalid_parentheses_r(set: HashSet<Vec<char>>) -> HashSet<Vec<char>> {
    if set.is_empty() {
        return HashSet::new();
    }

    let mut valid = HashSet::new();
    let mut invalid = HashSet::new();

    for s in set {
        for i in 0..s.len() {
            let mut s = s.clone();

            if s[i] == '(' || s[i] == ')' {
                s.remove(i);
            }

            if is_valid(&s) {
                valid.insert(s.clone());
            } else {
                invalid.insert(s.clone());
            }
        }
    }

    if valid.is_empty() {
        return remove_invalid_parentheses_r(invalid);
    }

    valid
}

fn is_valid(s: &[char]) -> bool {
    let mut opens = 0;
    let mut closes = 0;

    for c in s {
        match *c {
            '(' => opens += 1,
            ')' => closes += 1,
            _ => {},
        }

        if closes > opens {
            return false;
        }
    }

    opens == closes
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::problem_0301::remove_invalid_parentheses;

    #[test]
    fn test_1() {
        assert_eq!(
            remove_invalid_parentheses("()())()"),
            ["(())()", "()()()"]
                .into_iter()
                .map(ToString::to_string)
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            remove_invalid_parentheses("n"),
            ["n"]
                .into_iter()
                .map(ToString::to_string)
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            remove_invalid_parentheses("()"),
            ["()"]
                .into_iter()
                .map(ToString::to_string)
                .collect::<HashSet<_>>()
        );
    }
}
