fn eval_rpn<T>(tokens: &[T]) -> i32
where
    T: AsRef<str>,
{
    let mut stack = Vec::with_capacity(tokens.len().div_ceil(2));

    for token in tokens {
        if let Ok(digit) = token.as_ref().parse::<i32>() {
            stack.push(digit);
        } else {
            let second = stack.pop().unwrap();
            let first = stack.pop().unwrap();

            stack.push(match token.as_ref() {
                "+" => first + second,
                "-" => first - second,
                "*" => first * second,
                "/" => first / second,
                _ => unreachable!(),
            });
        }
    }

    stack.pop().unwrap()
}

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        eval_rpn(&tokens)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0150::eval_rpn;

    #[test]
    fn test_1() {
        assert_eq!(eval_rpn(&["2", "1", "+", "3", "*"]), 9);
    }

    #[test]
    fn test_2() {
        assert_eq!(eval_rpn(&["4", "13", "5", "/", "+"]), 6);
    }
}
