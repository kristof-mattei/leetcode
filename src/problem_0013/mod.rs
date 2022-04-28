fn when_letter_add(
    num: &mut i32,
    index: &mut usize,
    letters: &[char],
    in_decimal: i32,
    in_roman: &[char],
) {
    let len = in_roman.len();

    while letters
        .get(*index..(*index + len))
        .map_or(false, |s| s == in_roman)
    {
        *index += len;
        *num += in_decimal;
    }
}

fn roman_to_int(s: &str) -> i32 {
    let mut num = 0;
    let mut index: usize = 0;
    let chars = s.chars().collect::<Vec<_>>();
    when_letter_add(&mut num, &mut index, &chars, 1000, &['M']);
    when_letter_add(&mut num, &mut index, &chars, 900, &['C', 'M']);
    when_letter_add(&mut num, &mut index, &chars, 500, &['D']);
    when_letter_add(&mut num, &mut index, &chars, 400, &['C', 'D']);
    when_letter_add(&mut num, &mut index, &chars, 100, &['C']);
    when_letter_add(&mut num, &mut index, &chars, 90, &['X', 'C']);
    when_letter_add(&mut num, &mut index, &chars, 50, &['L']);
    when_letter_add(&mut num, &mut index, &chars, 40, &['X', 'L']);
    when_letter_add(&mut num, &mut index, &chars, 10, &['X']);
    when_letter_add(&mut num, &mut index, &chars, 9, &['I', 'X']);
    when_letter_add(&mut num, &mut index, &chars, 5, &['V']);
    when_letter_add(&mut num, &mut index, &chars, 4, &['I', 'V']);
    when_letter_add(&mut num, &mut index, &chars, 1, &['I']);

    num
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn roman_to_int(s: String) -> i32 {
        roman_to_int(&s)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0013::roman_to_int;

    #[test]
    fn test() {
        assert_eq!(roman_to_int("III"), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(roman_to_int("LVIII"), 58);
    }

    #[test]
    fn test_3() {
        assert_eq!(roman_to_int("MCMXCIV"), 1994);
    }
}
