use crate::shared::Solution;

fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut num = x;
    let mut reversed: i32 = 0;

    while num != 0 {
        reversed *= 10;
        reversed += num % 10;
        num /= 10;
    }

    reversed == x
}

impl Solution {
    #[must_use]
    pub fn is_palindrome(x: i32) -> bool {
        is_palindrome(x)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0009::is_palindrome;

    #[test]
    fn test_1() {
        assert!(is_palindrome(121));
    }
    #[test]
    fn test_2() {
        assert!(!is_palindrome(-121));
    }
    #[test]
    fn test_3() {
        assert!(!is_palindrome(10));
    }
}
