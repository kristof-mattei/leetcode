use crate::shared::Solution;

fn when_over_add(letters: &mut Vec<char>, num: &mut i32, over: i32, add: &[char]) {
    while *num >= over {
        add.iter().for_each(|c| letters.push(*c));

        *num -= over;
    }
}

fn int_to_roman(mut num: i32) -> String {
    let mut letters = Vec::new();

    when_over_add(&mut letters, &mut num, 1000, &['M']);
    when_over_add(&mut letters, &mut num, 900, &['C', 'M']);
    when_over_add(&mut letters, &mut num, 500, &['D']);
    when_over_add(&mut letters, &mut num, 400, &['C', 'D']);
    when_over_add(&mut letters, &mut num, 100, &['C']);
    when_over_add(&mut letters, &mut num, 90, &['X', 'C']);
    when_over_add(&mut letters, &mut num, 50, &['L']);
    when_over_add(&mut letters, &mut num, 40, &['X', 'L']);
    when_over_add(&mut letters, &mut num, 10, &['X']);
    when_over_add(&mut letters, &mut num, 9, &['I', 'X']);
    when_over_add(&mut letters, &mut num, 5, &['V']);
    when_over_add(&mut letters, &mut num, 4, &['I', 'V']);
    when_over_add(&mut letters, &mut num, 1, &['I']);

    letters.iter().collect()
}

impl Solution {
    #[must_use]
    pub fn int_to_roman(num: i32) -> String {
        int_to_roman(num)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0012::int_to_roman;

    #[test]
    fn test() {
        assert_eq!(int_to_roman(3), "III".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
    }
}
