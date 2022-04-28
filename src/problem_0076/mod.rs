use std::collections::{HashMap, HashSet, VecDeque};

fn min_window(s: &str, t: &str) -> String {
    if t.is_empty() {
        return String::new();
    }

    let target = t.bytes().fold(HashMap::<u8, usize>::new(), |mut h, e| {
        h.entry(e).and_modify(|v| *v += 1).or_insert(1);
        h
    });

    // when this one is empty we qualify
    // but we only remove an entry here once we have SURPASSED the target[c]'s count
    let mut qualify = t.bytes().collect::<HashSet<u8>>();

    let s_bytes = s.as_bytes();

    let mut ordered_match_indexes: VecDeque<usize> = VecDeque::new();
    let mut char_match_counts = HashMap::new();
    let mut return_value: &[u8] = &[];

    for (i, c) in s_bytes.iter().enumerate() {
        if !target.contains_key(c) {
            continue;
        }

        ordered_match_indexes.push_back(i);

        // how many entries of character c do we have left?
        if *char_match_counts
            .entry(c)
            .and_modify(|e| *e += 1)
            .or_insert(1)
            >= *target.get(c).unwrap()
        {
            // only when we have completely matched ALL counts of a character
            // do we remove it from qualify
            // makes it easier than every time
            // checking the char_match_counts against target
            qualify.remove(c);
        }

        // clean up any letters from the ordered_match_index
        // starting at 0, and remove anything that
        // has move uses than we need
        // but stop at the first one where we don't meet that criteria
        while !ordered_match_indexes.is_empty() {
            let matched_char = &s_bytes[ordered_match_indexes[0]];

            let cmc = char_match_counts.get_mut(matched_char).unwrap();

            if *cmc > *target.get(matched_char).unwrap() {
                ordered_match_indexes.pop_front();
                *cmc -= 1;
            } else {
                break;
            }
        }

        let last_index = ordered_match_indexes.len() - 1;

        if qualify.is_empty()
            && (return_value.is_empty()
                || ordered_match_indexes[last_index] - ordered_match_indexes[0] + 1
                    < return_value.len())
        {
            return_value = &s_bytes[ordered_match_indexes[0]..=ordered_match_indexes[last_index]];
        }
    }

    String::from_utf8(return_value.to_owned()).unwrap()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_window(s: String, t: String) -> String {
        min_window(&s, &t)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0076::min_window;

    #[test]
    fn test_1() {
        assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
    }

    #[test]
    fn test_2() {
        assert_eq!(min_window("a", "a"), "a");
    }

    #[test]
    fn test_3() {
        assert_eq!(min_window("a", "aa"), "");
    }

    #[test]
    fn test_4() {
        assert_eq!(min_window("a", "b"), "");
    }

    #[test]
    fn test_5() {
        assert_eq!(min_window("aa", "aa"), "aa");
    }

    #[test]
    fn test_6() {
        assert_eq!(
            min_window("ccabbccabcabcaacacaabacbcc", "cbbabbcacbc"),
            "bbccabcabcaacacaab"
        );
    }
}
