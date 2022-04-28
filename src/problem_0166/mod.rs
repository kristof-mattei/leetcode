use std::collections::HashMap;

use crate::shared::Solution;

fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    let result_is_negative = (numerator.signum() * denominator.signum()) < 0;

    let mut numerator = i64::from(numerator).abs();
    let denominator = i64::from(denominator).abs();

    let answer = if result_is_negative {
        format!("-{}", numerator / denominator)
    } else {
        (numerator / denominator).to_string()
    };

    numerator %= denominator;

    if numerator == 0 {
        return answer;
    }

    let mut remainders = HashMap::new();
    let mut after_period = vec![];

    while numerator != 0 {
        remainders.insert(numerator, after_period.len());
        numerator *= 10;

        after_period.push((((numerator / denominator) as u8) + b'0') as char);

        numerator %= denominator;

        if let Some(p) = remainders.get(&numerator) {
            let (non_repeating, repeating) = after_period.split_at(*p);

            return format!(
                "{}.{}({})",
                answer,
                &non_repeating.iter().collect::<String>(),
                &repeating.iter().collect::<String>()
            );
        }
    }

    format!("{}.{}", answer, &after_period.iter().collect::<String>())
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        fraction_to_decimal(numerator, denominator)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0166::fraction_to_decimal;

    #[test]
    fn test_1() {
        assert_eq!(fraction_to_decimal(1, 2), "0.5");
    }

    #[test]
    fn test_2() {
        assert_eq!(fraction_to_decimal(2, 1), "2");
    }

    #[test]
    fn test_3() {
        assert_eq!(fraction_to_decimal(4, 333), "0.(012)");
    }

    #[test]
    fn test_4() {
        assert_eq!(fraction_to_decimal(1, 6), "0.1(6)");
    }

    #[test]
    fn test_5() {
        assert_eq!(fraction_to_decimal(1, 333), "0.(003)");
    }

    #[test]
    fn test_6() {
        assert_eq!(fraction_to_decimal(1, 17), "0.(0588235294117647)");
    }

    #[test]
    fn test_7() {
        assert_eq!(fraction_to_decimal(-50, 8), "-6.25");
    }

    #[test]
    fn test_8() {
        assert_eq!(
            fraction_to_decimal(-1, -2_147_483_648),
            "0.0000000004656612873077392578125"
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(fraction_to_decimal(0, -3), "0");
    }

    #[test]
    fn test_10() {
        assert_eq!(fraction_to_decimal(0, 3), "0");
    }
}
