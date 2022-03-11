use crate::shared::Solution;

fn str_str(haystack: &str, needle: &str) -> i32 {
    let res = haystack.find(needle);

    res.map_or(-1, |r| r as i32)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        str_str(&haystack, &needle)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0028::str_str;

    #[test]
    fn test() {
        assert_eq!(str_str("hello", "ll"), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(str_str("aaaaa", "bba"), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(str_str("", ""), 0);
    }
}
