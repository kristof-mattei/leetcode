fn convert_to_title(mut column_number: i32) -> String {
    const CHARS_IN_ABC: i32 = 26;

    let mut buffer = vec![];

    while column_number > 0 {
        // because our range is A (1) -> Z (26), and we want to reset it to 0 -> 25.
        column_number -= 1;

        let remainder = column_number % CHARS_IN_ABC;

        // temp is now 0 - based ordinal, so we want to add whatever 'A' is to make 0 appear as 'A'
        buffer.push(((remainder as u8) + b'A') as char);

        column_number /= CHARS_IN_ABC;
    }

    buffer.iter().rev().collect()
}

impl Solution {
    #[must_use]
    pub fn convert_to_title(column_number: i32) -> String {
        convert_to_title(column_number)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0168::convert_to_title;

    #[test]
    fn test_1() {
        assert_eq!(convert_to_title(1), "A");
    }

    #[test]
    fn test_2() {
        assert_eq!(convert_to_title(28), "AB");
    }

    #[test]
    fn test_3() {
        assert_eq!(convert_to_title(701), "ZY");
    }

    #[test]
    fn test_4() {
        assert_eq!(convert_to_title(699), "ZW");
    }

    #[test]
    fn test_5() {
        assert_eq!(convert_to_title(703), "AAA");
    }

    #[test]
    fn test_6() {
        assert_eq!(convert_to_title(i32::MAX), "FXSHRXW");
    }

    #[test]
    fn test_7() {
        assert_eq!(convert_to_title(52), "AZ");
    }
}
