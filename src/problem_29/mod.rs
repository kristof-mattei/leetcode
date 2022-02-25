use crate::shared::Solution;

fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
    let mut quotient = 0;

    match (dividend, divisor) {
        (i32::MIN, i32::MIN) => return 1,
        (0, _) | (_, i32::MIN) => return 0,
        (i32::MIN, -1) => return i32::MAX,
        (i32::MIN, 1) => return i32::MIN,
        (i32::MIN, 2) => return i32::MIN >> 1,
        (i32::MIN, -2) => return -(i32::MIN >> 1),
        (i32::MIN, _) => {
            quotient += 1;
            if divisor < 0 {
                dividend -= divisor;
            }
            if divisor > 0 {
                dividend += divisor;
            }
        },
        _ => {},
    }

    let output_is_negative = dividend >> 31 != divisor >> 31;

    dividend = dividend.abs();
    divisor = divisor.abs();

    let shift = (divisor
        .leading_zeros()
        .checked_sub(dividend.leading_zeros()))
    .unwrap_or_default();

    for i in (0..=shift).rev() {
        if (divisor << i) <= dividend {
            dividend -= divisor << i;
            quotient += 1 << i;
        }
    }

    if output_is_negative {
        -quotient
    } else {
        quotient
    }
}

impl Solution {
    #[must_use]
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        divide(dividend, divisor)
    }
}

#[cfg(test)]
mod test {
    use crate::problem_29::divide;

    #[test]
    fn test() {
        assert_eq!(divide(10, 3), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(divide(7, -3), -2);
    }

    #[test]
    fn test_3() {
        assert_eq!(divide(0, -3), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(divide(1, 1), 1);
    }

    #[test]
    fn test_5() {
        assert_eq!(divide(i32::MIN, -3), 715_827_882);
    }

    #[test]
    fn test_6() {
        assert_eq!(divide(i32::MIN, 4), -536_870_912);
    }

    #[test]
    fn test_7() {
        assert_eq!(divide(1, 2), 0);
    }
}
