use crate::shared::Solution;

fn multiply(num1: &str, num2: &str) -> String {
    let mut digits = vec![0; num1.len() + num2.len()];

    let n1_stream = num1
        .chars()
        .rev()
        .enumerate()
        .map(|(i, ch)| (i, ch as u8 - b'0'));

    let n2_stream = num2
        .chars()
        .rev()
        .enumerate()
        .map(|(i, ch)| (i, ch as u8 - b'0'));

    for (index1, n1) in n1_stream {
        for (index2, n2) in n2_stream.clone() {
            let result = n1 * n2 + digits[index1 + index2];
            digits[index1 + index2] = result % 10;
            digits[index1 + index2 + 1] += result / 10;
        }
    }

    let mut i = digits.len() - 1;
    while i > 0 {
        if digits[i] == 0 {
            i -= 1;
        } else {
            break;
        }
    }

    String::from_utf8(
        digits[..=i]
            .iter()
            .map(|v| v + b'0')
            .rev()
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn multiply(num1: String, num2: String) -> String {
        multiply(&num1, &num2)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_43::multiply;

    #[test]
    fn test() {
        assert_eq!(multiply("2", "3"), "6".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(multiply("123", "456"), "56088".to_string());
    }
}
