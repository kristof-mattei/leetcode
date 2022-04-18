use std::fmt::Display;

use crate::shared::Solution;
enum Sign {
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn try_to_sign<T: AsRef<str>>(t: &T) -> Option<Sign> {
    match t.as_ref() {
        "+" => Sign::Plus.into(),
        "-" => Sign::Minus.into(),
        "*" => Sign::Multiply.into(),
        "/" => Sign::Divide.into(),
        _ => None,
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sign::Plus => write!(f, "+"),
            Sign::Minus => write!(f, "-"),
            Sign::Multiply => write!(f, "*"),
            Sign::Divide => write!(f, "/"),
        }
    }
}

fn eval_rpn<T>(tokens: &[T]) -> i32
where
    T: AsRef<str>,
    // [T]: Copy,
{
    let mut i = 0;

    let mut stack = vec![];
    loop {
        if let Some(sign) = try_to_sign(&tokens[i]) {
            // let op1 = String::from_utf8_lossy(tokens[i - 2].as_ref().as_bytes());
            // let op2 = String::from_utf8_lossy(tokens[i - 1].as_ref().as_bytes());

            // println!("{op1} {sign} {op2}");

            break;
        } else {
            stack.push(tokens[i]);
        }

        i += 1;
    }

    0
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        eval_rpn(&tokens)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0150::eval_rpn;

    #[test]
    fn test_1() {
        assert_eq!(eval_rpn(&["2", "1", "+", "3", "*"]), 9);
    }
}
