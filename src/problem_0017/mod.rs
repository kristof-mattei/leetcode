fn letter_combinations(digits: &str) -> Vec<String> {
    let numbers: Vec<_> = digits.chars().collect();

    match letter_combinations_rec(&numbers) {
        Some(combinations) => combinations.iter().map(|a| a.iter().collect()).collect(),
        None => vec![],
    }
}

fn letter_combinations_rec(numbers: &[char]) -> Option<Vec<Vec<char>>> {
    if numbers.is_empty() {
        return None;
    }

    let mut result = Vec::new();
    let letters = get_letters(numbers[0]);

    let combos = letter_combinations_rec(&numbers[1..]).unwrap_or_else(|| vec![vec![]]);

    for combo in combos {
        for letter in &letters {
            let mut n = vec![*letter];

            n.append(&mut combo.clone());

            result.push(n);
        }
    }

    Some(result)
}

fn get_letters(number_char: char) -> Vec<char> {
    match number_char.to_digit(10).unwrap() {
        2 => vec!['a', 'b', 'c'],
        3 => vec!['d', 'e', 'f'],
        4 => vec!['g', 'h', 'i'],
        5 => vec!['j', 'k', 'l'],
        6 => vec!['m', 'n', 'o'],
        7 => vec!['p', 'q', 'r', 's'],
        8 => vec!['t', 'u', 'v'],
        9 => vec!['w', 'x', 'y', 'z'],
        _ => unreachable!(),
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        letter_combinations(&digits)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0017::letter_combinations;

    #[test]
    fn test_2() {
        assert_eq!(letter_combinations(""), Vec::<String>::new());
    }

    #[test]
    fn test_3() {
        let mut result = letter_combinations("23");

        result.sort_unstable();
        assert_eq!(
            result,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
