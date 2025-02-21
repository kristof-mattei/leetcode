impl Solution {
    #[must_use]
    pub fn add_digits(num: i32) -> i32 {
        add_digits(num)
    }
}

fn add_digits(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }

    let result = num % 9;

    if result == 0 { 9 } else { result }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0258::add_digits;

    #[test]
    fn test_1() {
        assert_eq!(add_digits(38), 2);
    }
}
