use std::collections::HashSet;

impl Solution {
    #[must_use]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        nums.into_iter().any(|num| !set.insert(num))
    }
}

pub struct Solution;
