fn insert_delimiter(s: &str) -> Vec<char> {
    let ss = s;
    let mut sss = vec!['^', '£'];
    for i in ss.chars() {
        sss.push(i);
        sss.push('£');
    }
    sss.push('$');
    sss
}

fn manachar(s: &str) -> Vec<usize> {
    use std::cmp;
    let new_word: Vec<char> = insert_delimiter(s);
    let new_len = new_word.len();
    let mut new_vec = vec![0; new_len];
    let (mut r, mut c) = (0, 0);
    for i in 1..(new_len - 1) {
        let m_i = 2 * c - i;
        if r > i {
            new_vec[i] = cmp::min(r - i, new_vec[m_i]);
        } else {
            new_vec[i] = 0;
        }
        while new_word[i + 1 + new_vec[i]] == new_word[i - 1 - new_vec[i]] {
            new_vec[i] += 1;
        }
        if (i + new_vec[i]) > r {
            r = i + new_vec[i];
            c = i;
        }
    }
    new_vec
}

fn shortest_palindrome(s: &str) -> String {
    let n = insert_delimiter(s).len();
    let mut len = 1;
    let max_palind = manachar(s);
    for i in 1..n - 1 {
        if i - max_palind[i] <= 2 {
            len = max_palind[i];
        }
    }
    let strs: String = s[len..].chars().rev().collect();
    strs + s
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]

    pub fn shortest_palindrome(s: String) -> String {
        shortest_palindrome(&s)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::shortest_palindrome;

    #[test]
    fn test_1() {
        assert_eq!(shortest_palindrome("aacecaaa"), "aaacecaaa");
    }

    #[test]
    fn test_2() {
        assert_eq!(shortest_palindrome("abcd"), "dcbabcd");
    }
}
