fn my_atoi(mut s: &str) -> i32 {
    let mut i: u32 = 0;

    s = s.trim_start();
    if s.is_empty() {
        return 0;
    }

    let explicit_sign = match &s[..1] {
        "-" => Some(true),
        "+" => Some(false),
        _ => None,
    };

    let mut chars = s.chars();

    if explicit_sign.is_some() {
        chars.next();
    }

    for c in chars {
        let parsed = c.to_digit(10);
        match parsed {
            Some(digit) => {
                if let Some(t) = i.checked_mul(10) {
                    i = t;
                } else {
                    i = u32::MAX;
                    break;
                }
                i += digit;
            },
            None => {
                break;
            },
        }
    }

    let to_i32 = i32::try_from(i);

    match (explicit_sign, to_i32) {
        (Some(true), Ok(x)) => -x,
        (Some(true), Err(_)) => i32::MIN,
        (_, Ok(x)) => x,
        (_, Err(_)) => i32::MAX,
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn my_atoi(s: String) -> i32 {
        my_atoi(&s)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0008::my_atoi;

    #[test]
    fn test_1() {
        assert_eq!(my_atoi("42"), 42);
    }

    #[test]
    fn test_2() {
        assert_eq!(my_atoi("           -42"), -42);
    }

    #[test]
    fn test_3() {
        assert_eq!(my_atoi("4193 with words"), 4193);
    }
}
