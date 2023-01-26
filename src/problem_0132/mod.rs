fn min_cut(s: &[u8]) -> i32 {
    let len = s.len();
    let mut dp = (0..=len).collect::<Vec<_>>();

    for i in 0..len {
        for (l, r) in (0..=(usize::min(i, len - i - 1)))
            .map(|j| (i - j, i + j))
            .take_while(|&(l, r)| s[l] == s[r])
        {
            dp[r + 1] = usize::min(dp[r + 1], dp[l] + 1);
        }

        for (l, r) in (0..(usize::min(i + 1, len - i - 1)))
            .map(|j| (i - j, i + 1 + j))
            .take_while(|&(l, r)| s[l] == s[r])
        {
            dp[r + 1] = usize::min(dp[r + 1], dp[l] + 1);
        }
    }

    (dp[s.len()] - 1) as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_cut(s: String) -> i32 {
        min_cut(s.as_bytes())
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    use crate::problem_0132::min_cut;

    #[test]
    fn test_1() {
        assert_eq!(min_cut("aab".as_bytes()), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(min_cut("a".as_bytes()), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(min_cut("ab".as_bytes()), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(min_cut("aaabaa".as_bytes()), 1);
    }
}
