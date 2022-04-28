fn is_interleave(s1: &str, s2: &str, s3: &str) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    is_interleave_r(
        &mut vec![vec![None; s2.len()]; s1.len()],
        s1.as_bytes(),
        s2.as_bytes(),
        s3.as_bytes(),
    )
}

fn is_interleave_r(cache: &mut [Vec<Option<bool>>], s1: &[u8], s2: &[u8], s3: &[u8]) -> bool {
    if s1.is_empty() {
        return s2 == s3;
    }
    if s2.is_empty() {
        return s1 == s3;
    }

    if let Some(cached) = cache[s1.len() - 1][s2.len() - 1] {
        return cached;
    }

    let result = s1[0] == s3[0] && is_interleave_r(cache, &s1[1..], s2, &s3[1..])
        || s2[0] == s3[0] && is_interleave_r(cache, s1, &s2[1..], &s3[1..]);

    cache[s1.len() - 1][s2.len() - 1] = Some(result);

    result
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        is_interleave(&s1, &s2, &s3)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0097::is_interleave;

    #[test]
    fn test_1() {
        assert!(is_interleave("aabcc", "dbbca", "aadbbcbcac"));
    }

    #[test]
    fn test_2() {
        assert!(!is_interleave("aabcc", "dbbca", "aadbbcbccc"));
    }

    #[test]
    fn test_3() {
        assert!(is_interleave("", "", ""));
    }
}
