use crate::shared::Solution;

#[allow(dead_code)]
fn generate_parenthesis_r(n: i32, opens: i32, closes: i32) -> Vec<Vec<u8>> {
    if n == opens && n == closes {
        return vec![vec![]];
    }
    let mut result = Vec::new();
    if opens < n {
        for mut part in generate_parenthesis_r(n, opens + 1, closes) {
            let mut new = vec![b'('];
            new.append(&mut part);
            result.push(new);
        }
    }

    if closes < n {
        for mut part in generate_parenthesis_r(n, opens, closes + 1) {
            let mut new = vec![b')'];
            new.append(&mut part);
            result.push(new);
        }
    }

    result
}

fn generate_parenthesis_reversed_r(n: i32, opens: i32, closes: i32) -> Vec<String> {
    if n == opens && n == closes {
        return vec!["".to_string()];
    }

    let mut result = Vec::new();
    if opens < n {
        // it looks like we're closing one here, but upside down, its opening,
        // as pushing to a vec is easier than adding in front
        for mut part in generate_parenthesis_reversed_r(n, opens + 1, closes) {
            // mirrored version, so instead of prepending with ( we append )
            // in the end it's the same
            part.push(')');
            result.push(part);
        }
    }

    if closes < opens {
        // it looks like we're opening one here, but upside down, its closing,
        // as pushing to a vec is easier than adding in front
        for mut part in generate_parenthesis_reversed_r(n, opens, closes + 1) {
            // mirrored version, so instead of prepending with ) we append (
            // in the end it's the same
            part.push('(');
            result.push(part);
        }
    }

    result
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    generate_parenthesis_reversed_r(n, 0, 0)
}

impl Solution {
    #[must_use]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        generate_parenthesis(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_22::generate_parenthesis, shared::vec_eq};

    #[test]
    fn test() {
        assert!(vec_eq(generate_parenthesis(1), vec!["()".to_string()]));
    }

    #[test]
    fn test_2() {
        assert!(vec_eq(
            generate_parenthesis(3),
            vec![
                "((()))".to_string(),
                "()()()".to_string(),
                "(()())".to_string(),
                "()(())".to_string(),
                "(())()".to_string(),
            ]
        ));
    }
}
