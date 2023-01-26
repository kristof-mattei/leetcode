use std::collections::HashMap;
use std::{cmp, hash};

type Cache = HashMap<(Vec<char>, Vec<char>), Vec<Vec<char>>>;

fn permute_m(cache: &mut Cache, from: &[char], to: &[char]) -> Vec<Vec<char>> {
    let key = (from.to_vec(), to.to_vec());

    if cache.contains_key(&key) {
        // println!("Match found: {:?}", &key);
        return cache[&key].clone();
    }

    let r = permute(cache, from, to);

    // println!("Match inserted: {:?}", &key);
    cache.insert(key, r.clone());

    r
}

fn count_elements<T>(slice: &[T]) -> HashMap<T, usize>
where
    T: cmp::Eq + hash::Hash + Copy,
{
    slice.iter().fold(HashMap::new(), |mut map, c| {
        *map.entry(*c).or_insert(0) += 1;
        map
    })
}

fn get_pairs(
    from_left: &[char],
    from_right: &[char],
    to_left: &[char],
    to_right: &[char],
) -> (bool, bool) {
    let from_left_hash = count_elements(from_left);
    let from_right_hash = count_elements(from_right);

    let to_left_hash = count_elements(to_left);
    let to_right_hash = count_elements(to_right);

    let normal = (from_left_hash == to_left_hash) && (from_right_hash == to_right_hash);

    let swapped = (from_left_hash == to_right_hash) && (from_right_hash == to_left_hash);

    (normal, swapped)
}

fn permute(cache: &mut Cache, from: &[char], to: &[char]) -> Vec<Vec<char>> {
    if from == to {
        return vec![from.to_owned()];
    }

    let mut possibilities = vec![];
    for i in 1..from.len() {
        let (from_left, from_right) = from.split_at(i);
        {
            let (to_left, to_right) = to.split_at(i);

            let (normal, swapped) = get_pairs(from_left, from_right, to_left, to_right);

            if normal {
                let left_permutes = permute_m(cache, from_left, to_left);
                let right_permutes = permute_m(cache, from_right, to_right);

                let left_match = left_permutes.iter().any(|lp| lp == to_left);
                let right_match = right_permutes.iter().any(|rp| rp == to_right);

                if left_match && right_match {
                    println!("TO(NORMAL), NORMAL");
                    possibilities.push(to.to_vec());
                    break;
                }
            }

            if swapped {
                let left_permutes = permute_m(cache, from_left, to_right);
                let right_permutes = permute_m(cache, from_right, to_left);

                let left_match = left_permutes.iter().any(|lp| lp == to_right);
                let right_match = right_permutes.iter().any(|rp| rp == to_left);

                if left_match && right_match {
                    println!("TO(NORMAL), SWAPPED");
                    possibilities.push(to.to_vec());
                    break;
                }
            }
        }
        {
            let (to_left, to_right) = to.split_at(to.len() - i);

            let (normal, swapped) = get_pairs(from_left, from_right, to_left, to_right);

            if normal {
                let left_permutes = permute_m(cache, from_left, to_left);
                let right_permutes = permute_m(cache, from_right, to_right);

                let left_match = left_permutes.iter().any(|lp| lp == to_left);
                let right_match = right_permutes.iter().any(|rp| rp == to_right);

                if left_match && right_match {
                    println!("TO(REVERSED), NORMAL");
                    possibilities.push(to.to_vec());
                    break;
                }
            }

            if swapped {
                let left_permutes = permute_m(cache, from_left, to_right);
                let right_permutes = permute_m(cache, from_right, to_left);

                let left_match = left_permutes.iter().any(|lp| lp == to_right);
                let right_match = right_permutes.iter().any(|rp| rp == to_left);

                if left_match && right_match {
                    println!("TO(REVERSED), SWAPPED");
                    possibilities.push(to.to_vec());
                    break;
                }
            }
        }
    }

    possibilities
}

fn is_scramble(s1: &str, s2: &str) -> bool {
    let s2_vec = s2.chars().collect::<Vec<_>>();
    let permutations = permute(
        &mut HashMap::new(),
        &s1.chars().collect::<Vec<_>>(),
        &s2_vec,
    );

    permutations.iter().any(|p| p == &s2_vec)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_scramble_vec_char_char(s1: String, s2: String) -> bool {
        is_scramble(&s1, &s2)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0087::vec_char_char::is_scramble;

    #[test]
    fn test_1() {
        assert!(is_scramble("foobar", "foobar"));
    }

    #[test]
    fn test_2() {
        assert!(is_scramble("abcdbdacbdac", "bdacabcdbdac"));
    }

    #[test]
    fn test_3() {
        assert!(is_scramble("abcd", "cdba"));
    }

    #[test]
    fn test_4() {
        assert!(is_scramble("abb", "bba"));
    }
}
