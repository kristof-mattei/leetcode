use std::collections::HashMap;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut counts = HashMap::<[u8; 26], usize>::new();
    let mut anagrams = Vec::<Vec<String>>::new();

    for s in strs {
        let mut lc = [0; 26];
        for c in s.as_bytes() {
            lc[(c - b'a') as usize] += 1;
        }

        if let Some(&i) = counts.get(&lc) {
            anagrams[i].push(s);
        } else {
            anagrams.push(vec![s]);
            counts.insert(lc, anagrams.len() - 1);
        }
    }

    anagrams
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        group_anagrams(strs)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use crate::{problem_0049::group_anagrams, shared::sort_vec_of_vec};

    fn test_base(input: Vec<&str>, expected: Vec<Vec<&str>>) {
        let input = input
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        let mut expected = expected
            .into_iter()
            .map(|v| v.into_iter().map(ToString::to_string).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut result = group_anagrams(input);

        sort_vec_of_vec(&mut expected);
        sort_vec_of_vec(&mut result);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_1() {
        let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"];

        let expected = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];

        test_base(input, expected);
    }

    #[test]
    fn test_2() {
        let input = vec!["aza", "zaa"];

        let expected = vec![vec!["aza", "zaa"]];

        test_base(input, expected);
    }
}
