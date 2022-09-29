use std::cmp;

fn longest_valid_parentheses(s: &str) -> i32 {
    let mut max_valid = 0;
    let mut stack = vec![-1];

    for (index, symbol) in s.as_bytes().iter().enumerate() {
        let index = index as i32;
        match symbol {
            | b'(' => stack.push(index),
            | b')' => {
                stack.pop();

                if stack.is_empty() {
                    stack.push(index);
                } else {
                    max_valid = cmp::max(max_valid, index - stack[stack.len() - 1]);
                }
            },
            | _ => unreachable!(),
        }
    }

    max_valid
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn longest_valid_parentheses(s: String) -> i32 {
        longest_valid_parentheses(&s)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0032::longest_valid_parentheses;
    #[test]
    fn test_x() {
        let input = "()".to_string();
        assert_eq!(longest_valid_parentheses(&input), 2);
    }
    #[test]
    fn test() {
        let input = "(()".to_string();
        assert_eq!(longest_valid_parentheses(&input), 2);
    }

    #[test]
    fn test_2() {
        let input = ")()())".to_string();
        assert_eq!(longest_valid_parentheses(&input), 4);
    }

    #[test]
    fn test_3() {
        let input = "".to_string();
        assert_eq!(longest_valid_parentheses(&input), 0);
    }

    #[test]
    fn test_4() {
        let input = "()(())".to_string();
        assert_eq!(longest_valid_parentheses(&input), 6);
    }

    #[test]
    fn test_5() {
        let input = "()(()".to_string();
        assert_eq!(longest_valid_parentheses(&input), 2);
    }

    #[test]
    fn test_6() {
        let input = "(())(".to_string();
        assert_eq!(longest_valid_parentheses(&input), 4);
    }
}
