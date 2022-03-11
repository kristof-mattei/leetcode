use crate::shared::Solution;

fn is_palindrome(substring: &str) -> bool {
    let reversed = substring.as_bytes().iter().rev();
    let normal = substring.as_bytes();

    reversed.eq(normal)
}

fn find_longest(substring: &str, current_longest: usize) -> usize {
    for i in (current_longest..=(substring.len())).rev() {
        if i < current_longest {
            return 0;
        }
        if is_palindrome(&substring[0..i]) {
            return i;
        }
    }

    0
}

fn longest_palindrome(s: &str) -> String {
    let mut longest = "";

    for i in 0..s.len() {
        if s.len() - i < longest.len() {
            break;
        }
        let last_index_that_is_palindrome = find_longest(&s[i..], longest.len());

        let current = &s[i..(i + last_index_that_is_palindrome)];

        if current.len() > longest.len() {
            longest = current;
        }
    }
    longest.to_string()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn longest_palindrome(s: String) -> String {
        longest_palindrome(&s)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0005::longest_palindrome;

    #[test]
    fn test_1() {
        assert_eq!(longest_palindrome("babad"), "bab");
    }

    #[test]
    fn test_2() {
        assert_eq!(longest_palindrome("cbbd"), "bb");
    }
}
