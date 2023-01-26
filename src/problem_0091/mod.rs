fn num_decodings(s: &str) -> i32 {
    let bytes = s.as_bytes();

    let mut dp: Vec<u32> = vec![0; s.len()];

    if bytes[0] != b'0' {
        dp[0] = 1;
    }

    for i in 1..s.len() {
        let current = bytes[i] - b'0';
        if current != 0 {
            dp[i] += dp[i - 1];
        }

        let previous = bytes[i - 1] - b'0';

        if (10..=26).contains(&(previous * 10 + current)) {
            dp[i] += if i >= 2 { dp[i - 2] } else { 1 }
        }
    }

    dp[s.len() - 1] as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn num_decodings(s: String) -> i32 {
        num_decodings(&s)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0091::num_decodings;

    #[test]
    fn test_1() {
        let input = "12";

        let expected = 2;

        let result = num_decodings(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_2() {
        let input = "226";

        let expected = 3;

        let result = num_decodings(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_3() {
        let input = "06";

        let expected = 0;

        let result = num_decodings(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_4() {
        let input = "11106";

        let expected = 2;

        let result = num_decodings(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_5() {
        let input = "27";

        let expected = 1;

        let result = num_decodings(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_6() {
        let input = "2611055971756562";

        let expected = 4;

        let result = num_decodings(input);

        assert_eq!(expected, result);
    }
}
