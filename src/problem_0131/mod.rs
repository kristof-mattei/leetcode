use crate::shared::Solution;

fn is_palindrome(chars: &[char]) -> bool {
    let mut start = 0;

    let mut end = chars.len() - 1;

    while start < end {
        let left = chars[start];

        let right = chars[end];

        if left == right {
            start += 1;
            end -= 1;
            continue;
        }

        return false;
    }

    true
}
fn partition(s: &str) -> Vec<Vec<String>> {
    let chars = s.chars().collect::<Vec<_>>();

    let mut result = vec![];

    for i in 0..chars.len() {
        let split = chars.as_slice().split_at(i);

        if is_palindrome(split.0) && is_palindrome(split.1) {
            result.push(vec![
                split.0.to_owned().collect::<String>(),
                split.1.to_owned().collect::<String>(),
            ]);
        }
    }

    result
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn partition_0131(s: String) -> Vec<Vec<String>> {
        partition(&s)
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_0131::partition, shared::sort_vec_of_vec};

    #[test]
    fn test_1() {
        let mut result = partition("aab");
        sort_vec_of_vec(&mut result);

        assert_eq!(result, [vec!["a", "a", "b"], vec!["aa", "b"]]);
    }

    #[test]
    fn test_2() {
        let mut result = partition("a");
        sort_vec_of_vec(&mut result);

        assert_eq!(result, [vec!["a"]]);
    }
}
