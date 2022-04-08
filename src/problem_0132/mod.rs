use std::collections::HashMap;

use crate::shared::Solution;

fn is_palindrome(chars: &[char]) -> bool {
    match chars {
        [] | [_] => true,
        [..] => {
            let mut start = 0;

            let mut end = chars.len() - 1;

            while start < end {
                if chars[start] == chars[end] {
                    start += 1;
                    end -= 1;
                    continue;
                }

                return false;
            }

            true
        },
    }
}

fn memoize<'a>(cache: &mut HashMap<&'a [char], usize>, chars: &'a [char]) -> usize {
    if cache.contains_key(chars) {
        cache[chars]
    } else {
        let r = split_up(cache, chars);

        cache.insert(chars, r);

        r
    }
}

fn split_up<'a>(cache: &mut HashMap<&'a [char], usize>, chars: &'a [char]) -> usize {
    let mut results = vec![];

    if is_palindrome(chars) {
        results.push(vec![chars]);
    }

    for i in 1..chars.len() {
        let (l, r) = chars.split_at(i);

        if !is_palindrome(l) {
            continue;
        }

        // recursively split up right
        for mut right_side_split in memoize(cache, r) {
            let mut temp = vec![l];

            temp.append(&mut right_side_split);
            results.push(temp);
        }
    }

    results
}

fn min_cut(s: &str) -> i32 {
    let chars = s.chars().collect::<Vec<_>>();

    let r = split_up(&mut HashMap::new(), &chars);
    let mut result = vec![];

    for rr in r {
        let mut temp = vec![];
        for rrr in rr {
            let s = rrr.iter().collect::<String>();
            temp.push(s);
        }

        result.push(temp);
    }
    let min = result.into_iter().map(|r| r.len()).min().unwrap();

    min as i32 - 1
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_cut(s: String) -> i32 {
        min_cut(&s)
    }
}

#[cfg(test)]
mod tests {

    use crate::problem_0132::min_cut;

    #[test]
    fn test_1() {
        assert_eq!(min_cut("aab"), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(min_cut("a"), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(min_cut("ab"), 1);
    }
}
