use crate::shared::Solution;

fn length_of_longest_substring(s: &str) -> i32 {
    let mut longest = 0;

    let mut start = 0;
    let mut end = 0;

    for (i, c) in s.chars().enumerate() {
        if let Some(p) = s[start..end].chars().position(|cc| cc == c) {
            start += p + 1;
        }

        end = i + 1;

        longest = longest.max(s[start..end].len());
    }

    longest as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        length_of_longest_substring(&s)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0003::length_of_longest_substring;

    #[test]
    fn test_1() {
        assert_eq!(length_of_longest_substring("abcabcbb"), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(length_of_longest_substring("bbbbb"), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(length_of_longest_substring("pwwkew"), 3);
    }
}
