use std::fmt::Write;

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value)]
    pub fn calculate(input: String) -> i32 {
        calculate(&input)
    }
}

fn calculate(s: &str) -> i32 {
    let tokens = tokenize(s);

    let tree = parse_subtree(&tokens);

    tree.1.solve()
}

#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    LeftParenteses,
    RightParenteses,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => f.write_str(&n.to_string()),
            Token::Plus => f.write_char('+'),
            Token::Minus => f.write_char('-'),
            Token::LeftParenteses => f.write_char('('),
            Token::RightParenteses => f.write_char(')'),
        }
    }
}

#[derive(Debug)]
enum Tree {
    Number(i32),
    Add(Box<Tree>, Box<Tree>),
    Subtract(Box<Tree>, Box<Tree>),
    Negate(Box<Tree>),
}
impl Tree {
    fn solve(self) -> i32 {
        match self {
            Tree::Number(n) => n,
            Tree::Add(left, right) => left.solve() + right.solve(),
            Tree::Subtract(left, right) => left.solve() - right.solve(),
            Tree::Negate(expr) => -expr.solve(),
        }
    }
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let chars = s.chars().collect::<Vec<char>>();

    let mut index = 0;

    while let Some(c) = chars.get(index) {
        let token = match c {
            char @ '0'..='9' => {
                let mut num = i32::from(*char as u8 - b'0');

                while let Some(next_digit @ '0'..='9') = chars.get(index + 1) {
                    num = num * 10 + i32::from(*next_digit as u8 - b'0');

                    index += 1;
                }

                Some(Token::Number(num))
            },
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '(' => Some(Token::LeftParenteses),
            ')' => Some(Token::RightParenteses),
            ' ' => None,
            _ => panic!("Invalid character"),
        };

        index += 1;

        if let Some(token) = token {
            tokens.push(token);
        }
    }

    tokens
}

fn parse_term(tokens: &[Token]) -> (usize, Tree) {
    let next = tokens.first();

    match next {
        Some(Token::Number(n)) => (1, Tree::Number(*n)),
        Some(Token::LeftParenteses) => {
            let (advanced, subtree) = parse_subtree(&tokens[1..]);

            match tokens.get(advanced + 1) {
                Some(Token::RightParenteses) => (),
                _ => panic!("Expected right paren"),
            }

            (advanced + 2, subtree)
        },
        Some(Token::Minus) => {
            let (advanced, subtree) = parse_term(&tokens[1..]);

            (advanced + 1, Tree::Negate(Box::new(subtree)))
        },
        _ => panic!("Invalid token"),
    }
}

fn parse_subtree(tokens: &[Token]) -> (usize, Tree) {
    let (mut index, mut left) = parse_term(tokens);

    // make sure the parser is left associative by solving until we hit parenteses (in reality, this line should never hit a number)
    while let Some(operation @ (Token::Plus | Token::Minus)) = tokens.get(index) {
        // index + 1 as we're skipping the current token
        let (advanced, right) = parse_term(&tokens[index + 1..]);

        left = match operation {
            Token::Plus => Tree::Add(left.into(), right.into()),
            Token::Minus => Tree::Subtract(left.into(), right.into()),
            _ => panic!("Invalid token"),
        };

        index += advanced + 1;
    }

    (index, left)
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0224::calculate;

    #[test]
    fn test_1() {
        assert_eq!(calculate("1 + 1"), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(calculate(" 2-1 + 2 "), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)"), 23);
    }
}
