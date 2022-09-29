fn reverse(mut x: i32) -> i32 {
    let mut r: i32 = 0;

    while x != 0 {
        r = match r.checked_mul(10) {
            | Some(x) => x,
            | None => {
                return 0;
            },
        };
        r += x % 10;
        x /= 10;
    }
    r
}

impl Solution {
    #[must_use]
    pub fn reverse(x: i32) -> i32 {
        reverse(x)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0007::reverse;

    #[test]
    fn test_1() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn test_3() {
        assert_eq!(reverse(120), 21);
    }
}
