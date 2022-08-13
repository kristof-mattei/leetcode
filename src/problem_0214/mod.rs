fn is_palindrome(bytes: &[u8]) -> bool {
    match bytes {
        [] | [_] => true,
        [..] => {
            let mut start = 0;

            let mut end = bytes.len() - 1;

            while start < end {
                if bytes[start] == bytes[end] {
                    start += 1;
                    end -= 1;
                    continue;
                }

                return false;
            }

            true
        },
    }
}

fn shortest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return String::from("");
    }
    let bytes = s.as_bytes();

    for i in (0..bytes.len()).rev() {
        if is_palindrome(&bytes[0..=i]) {
            let mut r = String::with_capacity(s.len() + s.len() - i);

            let mut rev = bytes[(i + 1)..].to_vec();
            rev.reverse();

            r.push_str(&String::from_utf8_lossy(&rev));

            r.push_str(s);

            return r;
        }
    }

    unreachable!()
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
