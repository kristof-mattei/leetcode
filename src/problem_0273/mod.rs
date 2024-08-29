impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn number_to_words(num: i32) -> String {
        number_to_words(num)
    }
}

const TENS: [&str; 10] = [
    "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

const NUMBERS: [&str; 20] = [
    "",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];

fn number_to_words_r(num: usize) -> String {
    let mut s = String::new();

    if num < 20 {
        s.push_str(NUMBERS[num]);
    } else if num < 100 {
        s.push_str(TENS[num / 10]);
        s.push(' ');
        s.push_str(NUMBERS[num % 10]);
    } else if num < 1000 {
        s.push_str(NUMBERS[num / 100]);
        s.push_str(" Hundred ");
        s.push_str(&number_to_words_r(num % 100));
    } else if num < 1_000_000 {
        s.push_str(&number_to_words_r(num / 1000));
        s.push_str(" Thousand ");
        s.push_str(&number_to_words_r(num % 1000));
    } else if num < 1_000_000_000 {
        s.push_str(&number_to_words_r(num / 1_000_000));
        s.push_str(" Million ");
        s.push_str(&number_to_words_r(num % 1_000_000));
    } else {
        s.push_str(&number_to_words_r(num / 1_000_000_000));
        s.push_str(" Billion ");
        s.push_str(&number_to_words_r(num % 1_000_000_000));
    }

    s.trim().to_string()
}

fn number_to_words(num: i32) -> String {
    if num == 0 {
        return String::from("Zero");
    }

    number_to_words_r(num as usize)
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0273::number_to_words;

    #[test]
    fn test_1() {
        assert_eq!(number_to_words(123), "One Hundred Twenty Three");
    }
}
