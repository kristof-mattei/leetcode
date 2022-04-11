use std::collections::{hash_map::Entry, HashMap};

use crate::shared::Solution;

// this one is cool too:
// https://leetcode.com/submissions/detail/678459562/

// let mut one = 0;
// let mut two = 0;
// let mut three = 0;

// for &n in &nums {
//     three = two & n;
//     two = two | (one & n);
//     one = one ^ n;
//     one &= !three;
//     two &= !three;
// }
// one

fn single_number(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();

    for n in nums {
        match hashmap.entry(n) {
            Entry::Vacant(v) => {
                v.insert(1);
            },
            Entry::Occupied(mut o) => {
                if *o.get() == 2 {
                    o.remove_entry();
                } else {
                    *o.get_mut() += 1;
                }
            },
        }
    }

    hashmap.into_iter().next().unwrap().0
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn single_number_0137(nums: Vec<i32>) -> i32 {
        single_number(nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0137::single_number;

    #[test]
    fn test_1() {
        assert_eq!(single_number(vec![2, 2, 3, 2]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
