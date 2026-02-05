impl Solution {
    #[must_use]
    pub fn is_ugly(n: i32) -> bool {
        is_ugly(n)
    }
}

fn is_ugly(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    while n % 5 == 0 {
        n /= 5;
    }
    while n % 3 == 0 {
        n /= 3;
    }
    while n % 2 == 0 {
        n /= 2;
    }

    n == 1
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0263::is_ugly;

    #[test]
    fn test_1() {
        assert!(is_ugly(6));
    }

    #[test]
    fn test_2() {
        assert!(is_ugly(1));
    }

    #[test]
    fn test_3() {
        assert!(!is_ugly(14));
    }
}
