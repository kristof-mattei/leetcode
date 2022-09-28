fn find_lps(s: &[char]) -> usize {
    let n = s.len();

    let mut lps = vec![0; n];

    let mut len = 0;
    let mut i = 1;

    while i < n {
        if s[len] == s[i] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }

    lps[n - 1]
}

fn shortest_palindrome(s: &str) -> String {
    let mut original = s.chars().collect::<Vec<char>>();

    // char length, not byte length
    let original_length = &original.len();

    original.push('#');
    original.extend(s.chars().rev());

    let length = find_lps(original.as_slice());

    format!(
        "{}{}",
        s.chars()
            .rev()
            .take(original_length - length)
            .collect::<String>(),
        s
    )
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn shortest_palindrome(s: String) -> String {
        shortest_palindrome(&s)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::shortest_palindrome;

    #[test]
    fn test_1() {
        assert_eq!(shortest_palindrome("aacecaaa"), "aaacecaaa");
    }

    #[test]
    fn test_2() {
        assert_eq!(shortest_palindrome("abcd"), "dcbabcd");
    }
}
