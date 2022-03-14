use std::{collections::HashMap, rc::Rc};

pub mod vec_char_char;

use crate::shared::Solution;

type L = u8;
type ValidSliceCache<'a> = HashMap<(&'a [L], &'a [L]), bool>;
type SliceCharCountCache<'a> = HashMap<&'a [L], Rc<HashMap<L, usize>>>;

fn permute_m<'a, 'b>(
    valid_slice_cache: &'a mut ValidSliceCache<'b>,
    slice_char_count_cache: &mut SliceCharCountCache<'b>,
    from: &'b [L],
    to: &'b [L],
) -> bool {
    if valid_slice_cache.contains_key(&(from, to)) {
        return valid_slice_cache[&(from, to)];
    }

    let r = permute(valid_slice_cache, slice_char_count_cache, from, to);

    valid_slice_cache.insert((from, to), r);

    r
}

fn count_elements<'a, 'b>(cache: &'a mut SliceCharCountCache<'b>, slice: &'b [L]) -> Rc<HashMap<L, usize>> {
    if cache.contains_key(slice) {
        return Rc::clone(&cache[slice]);
    }

    let r = slice.iter().fold(HashMap::new(), |mut map, c| {
        *map.entry(*c).or_insert(0) += 1;
        map
    });

    cache.insert(slice, Rc::new(r));

    Rc::clone(&cache[slice])
}

fn normal_or_swapped_or_both(
    from_left_hash: &HashMap<u8, usize>,
    from_right_hash: &HashMap<u8, usize>,
    to_left_hash: &HashMap<u8, usize>,
    to_right_hash: &HashMap<u8, usize>,
) -> (bool, bool) {
    let normal = (from_left_hash == to_left_hash) && (from_right_hash == to_right_hash);
    let swapped = (from_left_hash == to_right_hash) && (from_right_hash == to_left_hash);

    (normal, swapped)
}

fn permute<'a, 'b>(
    valid_slice_cache: &'a mut ValidSliceCache<'b>,
    slice_char_count_cache: &mut SliceCharCountCache<'b>,
    from: &'b [L],
    to: &'b [L],
) -> bool {
    if from.len() != to.len() {
        return false;
    }

    if from == to {
        return true;
    }

    for i in 1..from.len() {
        let (from_left, from_right) = from.split_at(i);
        let from_left_hash = count_elements(slice_char_count_cache, from_left);
        let from_right_hash = count_elements(slice_char_count_cache, from_right);

        for split_point in [i, to.len() - i] {
            let (to_left, to_right) = to.split_at(split_point);

            let to_left_hash = count_elements(slice_char_count_cache, to_left);
            let to_right_hash = count_elements(slice_char_count_cache, to_right);

            let (normal, swapped) = normal_or_swapped_or_both(
                &from_left_hash,
                &from_right_hash,
                &to_left_hash,
                &to_right_hash,
            );

            if normal
                && permute_m(
                    valid_slice_cache,
                    slice_char_count_cache,
                    from_left,
                    to_left,
                )
                && permute_m(
                    valid_slice_cache,
                    slice_char_count_cache,
                    from_right,
                    to_right,
                )
            {
                return true;
            }

            if swapped
                && permute_m(
                    valid_slice_cache,
                    slice_char_count_cache,
                    from_left,
                    to_right,
                )
                && permute_m(
                    valid_slice_cache,
                    slice_char_count_cache,
                    from_right,
                    to_left,
                )
            {
                return true;
            }
        }
    }

    false
}

fn is_scramble(s1: &str, s2: &str) -> bool {
    let permutations = permute(
        &mut HashMap::new(),
        &mut HashMap::new(),
        s1.as_bytes(),
        s2.as_bytes(),
    );

    permutations
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_scramble(s1: String, s2: String) -> bool {
        is_scramble(&s1, &s2)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0087::is_scramble;

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

    #[test]
    fn test_5() {
        assert!(is_scramble("abc", "cab"));
    }

    #[test]
    fn test_6() {
        assert!(is_scramble("hobobyrqd", "hbyorqdbo"));
    }
}
