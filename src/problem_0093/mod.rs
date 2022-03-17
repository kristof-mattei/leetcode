use crate::shared::Solution;

fn restore_ip_addresses_r(digits: &[char], octet: usize) -> Vec<Vec<Vec<char>>> {
    if octet == 5 {
        if digits.is_empty() {
            return vec![vec![]];
        }
        return vec![];
    }

    let mut results = vec![];

    let until = if digits[0] == '0' {
        1
    } else {
        3.min(digits.len())
    };

    for i in 1..=until {
        let (prefix, remainder) = digits.split_at(i);

        if prefix.len() == 3 && is_larger_than_255(prefix) {
            continue;
        }

        if 4 - octet > remainder.len() {
            continue;
        }

        let remainders = restore_ip_addresses_r(remainder, octet + 1);

        for r in remainders {
            results.push([vec![prefix.to_vec()], r].concat());
        }
    }

    results
}

fn is_larger_than_255(digits: &[char]) -> bool {
    let count = digits.iter().fold(0, |mut acc, &elem| {
        acc *= 10;
        acc += elem as u32 - '0' as u32;
        acc
    });

    count > 255
}

fn restore_ip_addresses(s: &str) -> Vec<String> {
    restore_ip_addresses_r(&s.chars().collect::<Vec<_>>(), 1)
        .into_iter()
        .map(|ip| {
            ip.into_iter()
                .map(|o| o.into_iter().collect::<String>())
                .collect::<Vec<_>>()
                .join(".")
        })
        .collect()
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

    #[test]
    fn test_4() {
        let input = "172162541";

        let expected = [
            "17.216.25.41",
            "17.216.254.1",
            "172.16.25.41",
            "172.16.254.1",
            "172.162.5.41",
            "172.162.54.1",
        ];

        assert_eq!(restore_ip_addresses(input), expected);
    }
}
