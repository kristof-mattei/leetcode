use crate::shared::Solution;

fn restore_ip_addresses(_s: &str) -> Vec<String> {
    // see 91
    vec![]
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        restore_ip_addresses(&s)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0093::restore_ip_addresses;

    #[test]
    fn test_1() {
        let input = "0000";

        let expected = ["0.0.0.0"];

        assert_eq!(restore_ip_addresses(input), expected);
    }
    #[test]
    fn test_2() {
        let input = "25525511135";

        let expected = ["255.255.11.135", "255.255.111.35"];

        assert_eq!(restore_ip_addresses(input), expected);
    }
    #[test]
    fn test_3() {
        let input = "101023";

        let expected = [
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3",
        ];

        assert_eq!(restore_ip_addresses(input), expected);
    }
}
