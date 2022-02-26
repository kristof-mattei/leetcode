use std::thread::current;

use crate::shared::Solution;

fn longest_valid_parentheses(s: String) -> i32 {
    let mut max_valid = 0;
    let mut current_valid = 0;
    let mut opens = 0;
    // let mut closes = 0;

    let bytes = s.as_bytes();

    for i in 0..s.len() {
        for symbol in bytes.iter().skip(i) {
            match symbol {
                b'(' => opens += 1,
                b')' => {
                    if opens == 0 {
                        break;
                    }
                    opens -= 1;
                },
                _ => unreachable!(),
            }
            if opens == 0 {
                current_valid += 1;
            }
        }
        if current_valid > max_valid {
            max_valid = current_valid;
        }
        // i += current_valid;
    }

    max_valid
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn longest_valid_parentheses(s: String) -> i32 {
        longest_valid_parentheses(s)
    }
}

#[cfg(test)]
mod test {
    use crate::problem_32::longest_valid_parentheses;

    #[test]
    fn test() {
        let input = "(()".to_string();
        assert_eq!(longest_valid_parentheses(input), 2);
    }
    #[test]
    fn test_2() {
        let input = ")()())".to_string();
        assert_eq!(longest_valid_parentheses(input), 4);
    }
    #[test]
    fn test_3() {
        let input = "".to_string();
        assert_eq!(longest_valid_parentheses(input), 0);
    }
}
