use std::iter::Peekable;
use std::vec::IntoIter;

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn calculate(input: String) -> i32 {
        calculate(&input)
    }
}

fn calculate(s: &str) -> i32 {
    let tokens = tokenize(s);

    let tree = parse_expression(&mut tokens.into_iter().peekable());

    tree.solve()
}

#[derive(Debug)]
enum Token {
    Number(i32),
    Divide,
    Times,
    Plus,
    Minus,
    LeftParenteses,
    RightParenteses,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Divide => write!(f, "/"),
            Token::Times => write!(f, "*"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::LeftParenteses => write!(f, "("),
            Token::RightParenteses => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
enum Tree {
    Number(i32),
    Times(Box<Tree>, Box<Tree>),
    Divide(Box<Tree>, Box<Tree>),
    Add(Box<Tree>, Box<Tree>),
    Subtract(Box<Tree>, Box<Tree>),
    Negate(Box<Tree>),
}

impl Tree {
    fn solve(self) -> i32 {
        match self {
            Tree::Number(n) => n,
            Tree::Times(left, right) => left.solve() * right.solve(),
            Tree::Divide(left, right) => left.solve() / right.solve(),
            Tree::Add(left, right) => left.solve() + right.solve(),
            Tree::Subtract(left, right) => left.solve() - right.solve(),
            Tree::Negate(expr) => -expr.solve(),
        }
    }
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        let token = match c {
            char @ '0'..='9' => {
                let mut num = i32::from(char as u8 - b'0');

                while let Some(next_digit) = chars.next_if(char::is_ascii_digit) {
                    num = num * 10 + i32::from(next_digit as u8 - b'0');
                }

                Some(Token::Number(num))
            },
            '/' => Some(Token::Divide),
            '*' => Some(Token::Times),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '(' => Some(Token::LeftParenteses),
            ')' => Some(Token::RightParenteses),
            ' ' => None,
            _ => panic!("Invalid character"),
        };

        if let Some(token) = token {
            tokens.push(token);
        }
    }

    tokens
}

fn parse_primary_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Tree {
    match tokens.next() {
        Some(Token::Number(n)) => Tree::Number(n),
        Some(Token::LeftParenteses) => {
            let subtree = parse_expression(tokens);

            match tokens.next() {
                Some(Token::RightParenteses) => (),
                _ => panic!("Expected right paren"),
            }

            subtree
        },
        Some(Token::Minus) => {
            let subtree = parse_primary_expression(tokens);

            Tree::Negate(Box::new(subtree))
        },

        _ => panic!("Invalid token"),
    }
}

fn parse_multiply_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Tree {
    let mut left = parse_primary_expression(tokens);

    while let Some(operation) = tokens.next_if(|t| matches!(*t, Token::Divide | Token::Times)) {
        // index + 1 as we're skipping the current token
        let right = parse_primary_expression(tokens);

        left = match operation {
            Token::Times => Tree::Times(left.into(), right.into()),
            Token::Divide => Tree::Divide(left.into(), right.into()),
            Token::Number(_)
            | Token::Plus
            | Token::Minus
            | Token::LeftParenteses
            | Token::RightParenteses => panic!("Invalid token"),
        };
    }

    left
}

fn parse_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Tree {
    let mut left = parse_multiply_expression(tokens);

    // make sure the parser is left associative by solving until we hit parenteses (in reality, this line should never hit a number)
    while let Some(operation) = tokens.next_if(|t| matches!(*t, Token::Plus | Token::Minus)) {
        // index + 1 as we're skipping the current token
        let right = parse_multiply_expression(tokens);

        left = match operation {
            Token::Plus => Tree::Add(left.into(), right.into()),
            Token::Minus => Tree::Subtract(left.into(), right.into()),
            Token::Number(_)
            | Token::Divide
            | Token::Times
            | Token::LeftParenteses
            | Token::RightParenteses => panic!("Invalid token"),
        };
    }

    left
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0227::calculate;

    #[test]
    fn test_1() {
        assert_eq!(calculate("3+2*2"), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(calculate(" 3/2 "), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(calculate(" 3+5 / 2 "), 5);
    }

    #[test]
    fn test_4() {
        assert_eq!(calculate("1-(     -2)"), 3);
    }

    #[test]
    fn test_5() {
        assert_eq!(calculate("2*3+4"), 10);
    }
}
