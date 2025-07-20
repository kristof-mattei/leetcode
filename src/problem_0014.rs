use std::string::String;

fn longest_common_prefix(strs: &[String]) -> String {
    let as_chars: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();

    let mut prefix = Vec::new();
    let mut index = 0;

    'outer: while let Some(char) = as_chars[0].get(index) {
        for ac in as_chars.iter().skip(1) {
            match ac.get(index) {
                Some(c) if c == char => {},
                _ => break 'outer,
            }
        }

        index += 1;
        prefix.push(char);
    }

    prefix.into_iter().collect::<String>()
}

/// this one only works on strings constructed with characters represented in one byte.
#[expect(dead_code)]
fn longest_common_prefix_a_z(strs: &[String]) -> String {
    let mut prefix = Vec::new();
    let mut index: usize = 0;

    'outer: while let Some(char) = strs[0].get(index..=index) {
        for ac in strs.iter().skip(1) {
            match ac.get(index..=index) {
                Some(c) if c == char => {},
                _ => break 'outer,
            }
        }

        index += 1;
        prefix.push(char);
    }

    prefix.into_iter().collect::<String>()
}

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        longest_common_prefix(&strs)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0014::longest_common_prefix;

    #[test]
    fn test() {
        assert_eq!(
            longest_common_prefix(
                ["flower", "flow", "flight"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .as_ref()
            ),
            "fl".to_owned()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            longest_common_prefix(
                ["dog", "racecar", "car"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .as_ref()
            ),
            String::new()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            longest_common_prefix(
                ["a"]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .as_ref()
            ),
            "a".to_owned()
        );
    }
}
