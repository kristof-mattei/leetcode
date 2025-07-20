impl Solution {
    #[must_use]
    pub fn is_power_of_three(n: i32) -> bool {
        is_power_of_three(n)
    }
}

const MAX_POWER_OF_3_IN_I32: i32 = 3_i32.pow(19);

fn is_power_of_three(n: i32) -> bool {
    n > 0 && MAX_POWER_OF_3_IN_I32 % n == 0

    // without loops
    // let exponent = f64::from(n).log(3.0);

    // if exponent == f64::INFINITY {
    //     return false;
    // }

    // 3i32.pow(exponent.round() as u32) == n
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0326::is_power_of_three;

    #[test]
    fn test_1() {
        assert!(is_power_of_three(9));
    }

    #[test]
    fn test_2() {
        assert!(is_power_of_three(27));
    }

    #[test]
    fn test_3() {
        assert!(!is_power_of_three(0));
    }

    #[test]
    fn test_4() {
        assert!(is_power_of_three(243));
    }
}
