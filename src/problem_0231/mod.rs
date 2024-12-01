impl Solution {
    #[must_use]
    pub fn is_power_of_two(n: i32) -> bool {
        is_power_of_two(n)
    }
}

fn is_power_of_two(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0
    // or n.count_ones() == 1
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0231::is_power_of_two;

    #[test]
    fn test_1() {
        assert!(is_power_of_two(4));
    }

    #[test]
    fn test_2() {
        assert!(!is_power_of_two(6));
    }
}
