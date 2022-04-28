fn parse(decimal: &str, allow_dots: bool) -> bool {
    let chars: Vec<char> = decimal.chars().collect();

    let mut dot_found = false;
    let mut number_found = false;

    for (p, &c) in chars.iter().enumerate() {
        match c {
            '-' | '+' => {
                if p != 0 {
                    return false;
                }
            },
            '0'..='9' => {
                number_found = true;
            },
            '.' => {
                if !allow_dots || dot_found {
                    return false;
                }
                dot_found = true;
            },
            _ => {
                return false;
            },
        }
    }

    number_found
}

fn is_number(s: &str) -> bool {
    let split = s.split_once(&['e', 'E']);

    match split {
        Some((l, r)) => parse(l, true) && parse(r, false),
        None => parse(s, true),
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_number(s: String) -> bool {
        is_number(&s)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0065::is_number;

    #[test]
    fn test_1() {
        assert!(is_number("0"));
    }

    #[test]
    fn test_2() {
        assert!(!is_number("e"));
    }

    #[test]
    fn test_3() {
        assert!(!is_number("."));
    }

    #[test]
    fn test_4() {
        assert!(!is_number("1e2e3"));
    }

    #[test]
    fn test_5() {
        assert!(!is_number("+."));
    }
}
