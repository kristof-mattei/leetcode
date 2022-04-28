use std::collections::HashMap;

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

fn memoize<'a>(
    cache: &mut HashMap<&'a [char], Vec<Vec<&'a [char]>>>,
    chars: &'a [char],
) -> Vec<Vec<&'a [char]>> {
    if cache.contains_key(chars) {
        cache[chars].clone()
    } else {
        let r = split_up(cache, chars);

        cache.insert(chars, r.clone());

        println!("Cache hit!");
        r
    }
}

fn split_up<'a>(
    cache: &mut HashMap<&'a [char], Vec<Vec<&'a [char]>>>,
    chars: &'a [char],
) -> Vec<Vec<&'a [char]>> {
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

fn partition(s: &str) -> Vec<Vec<String>> {
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
    result
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn partition(s: String) -> Vec<Vec<String>> {
        partition(&s)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    use crate::{problem_0131::partition, shared::sort_vec_of_vec};

    #[test]
    fn test_1() {
        let input = "aab";

        let expected = [vec!["a", "a", "b"], vec!["aa", "b"]];

        let mut result = partition(input);

        sort_vec_of_vec(&mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let input = "a";

        let expected = [vec!["a"]];

        let mut result = partition(input);

        sort_vec_of_vec(&mut result);
        assert_eq!(result, expected);
    }
}
