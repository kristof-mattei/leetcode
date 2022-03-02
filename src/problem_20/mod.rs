use crate::shared::Solution;

fn is_valid(s: &str) -> bool {
    // s.len() is valid for strings
    // which only use 1 byte characters
    let mut opens: Vec<char> = Vec::with_capacity(s.len());

    for symbol in s.chars() {
        match symbol {
            '(' => opens.push(')'),
            '[' => opens.push(']'),
            '{' => opens.push('}'),
            ')' | ']' | '}' => {
                if Some(symbol) != opens.pop() {
                    return false;
                }
            },
            _ => unreachable!(),
        }
    }

    opens.is_empty()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_valid(s: String) -> bool {
        is_valid(&s)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_20::is_valid;

    #[test]
    fn test() {
        assert!(is_valid("()"));
    }
}
