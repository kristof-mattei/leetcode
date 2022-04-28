fn remove_duplicate_letters(s: &str) -> String {
    let s_bytes = s.as_bytes();

    let mut char_count = [0; 26];
    let mut visited = [false; 26];

    let mut result = Vec::new();

    for i in 0..s.len() {
        char_count[(s_bytes[i] - b'a') as usize] += 1;
    }

    for i in 0..s.len() {
        char_count[(s_bytes[i] - b'a') as usize] -= 1;

        if !visited[(s_bytes[i] - b'a') as usize] {
            while result.last().map_or(false, |&v| {
                v > s_bytes[i] && char_count[(v - b'a') as usize] > 0
            }) {
                // marking letter visited
                visited[result.last().map(|v| v - b'a').unwrap() as usize] = false;
                result.pop();
            }

            // Add s[i] in res and
            // mark it visited
            result.push(s_bytes[i]);
            visited[(s_bytes[i] - b'a') as usize] = true;
        }
    }
    // return resultant string
    unsafe { String::from_utf8_unchecked(result) }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_duplicate_letters(s: String) -> String {
        remove_duplicate_letters(&s)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0316::remove_duplicate_letters;

    #[test]
    fn test_1() {
        assert_eq!(remove_duplicate_letters("bcabc"), "abc");
    }

    #[test]
    fn test_2() {
        assert_eq!(remove_duplicate_letters("cbacdcbc"), "acdb");
    }
}
