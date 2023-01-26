fn title_to_number(column_title: &str) -> i32 {
    // A = 65, but then A would be 0, so we only remove 64
    const A_IS_ONE: u8 = 64;

    column_title
        .as_bytes()
        .iter()
        .map(|b| i32::from(b - A_IS_ONE))
        .fold(0, |acc, curr| acc * 26 + curr)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn title_to_number(column_title: String) -> i32 {
        title_to_number(&column_title)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0171::title_to_number;

    #[test]
    fn test_1() {
        assert_eq!(title_to_number("A"), 1);
    }
    #[test]
    fn test_2() {
        assert_eq!(title_to_number("AB"), 28);
    }
    #[test]
    fn test_3() {
        assert_eq!(title_to_number("ZY"), 701);
    }
}
