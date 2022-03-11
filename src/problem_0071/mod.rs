use crate::shared::Solution;

fn simplify_path(path: &str) -> String {
    let pieces = path.split('/').collect::<Vec<_>>();

    let mut simplified = Vec::new();

    for piece in pieces {
        match piece {
            ".." => {
                simplified.pop();
            },
            "" | "." => {
                // ignore
            },
            p => {
                simplified.push(p);
            },
        }
    }

    if simplified.is_empty() {
        return "/".to_string();
    }

    simplified.into_iter().fold(String::new(), |mut acc, curr| {
        acc.push('/');
        acc.push_str(curr);
        acc
    })
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn simplify_path(path: String) -> String {
        simplify_path(&path)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0071::simplify_path;

    #[test]
    fn test_1() {
        assert_eq!(simplify_path("/home/"), "/home");
    }

    #[test]
    fn test_2() {
        assert_eq!(simplify_path("/../"), "/");
    }

    #[test]
    fn test_3() {
        assert_eq!(simplify_path("/home//foo/"), "/home/foo");
    }
}
