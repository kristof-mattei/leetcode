impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn main_fn(input: Vec<String>) -> i32 {
        sub_fn(&input)
    }
}

fn sub_fn(input: &[String]) -> i32 {
    input.len() as i32
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0229::sub_fn;

    #[test]
    fn test_1() {
        assert_eq!(
            sub_fn(
                &["a", "b", "c", "d", "e"]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
            ),
            5
        );
    }
}
