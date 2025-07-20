impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn missing_number(input: Vec<i32>) -> i32 {
        missing_number(&input)
    }
}

fn missing_number(input: &[i32]) -> i32 {
    let len = input.len() as i32;

    // https://en.wikipedia.org/wiki/Triangular_number
    let mut sum = i32::midpoint(len * len, len);

    for i in input {
        sum -= *i;
    }
    sum
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0268::missing_number;

    #[test]
    fn test_1() {
        assert_eq!(missing_number(&[3, 0, 1]), 2);
    }
}
