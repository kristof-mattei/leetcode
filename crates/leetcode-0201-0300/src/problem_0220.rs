use std::collections::BTreeSet;

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        contains_nearby_almost_duplicate(&nums, index_diff, value_diff)
    }
}

fn contains_nearby_almost_duplicate(nums: &[i32], index_diff: i32, value_diff: i32) -> bool {
    let index_diff = index_diff as usize;

    let mut set: BTreeSet<i32> = BTreeSet::new();

    for (i, num) in nums.iter().enumerate() {
        let lower_bound = num - value_diff;
        let upper_bound = num + value_diff;

        if set.range(lower_bound..=upper_bound).next().is_some() {
            return true;
        }

        // Insert the current element into the set
        set.insert(*num);

        // Prune lower range
        if let Some(lower) = i.checked_sub(index_diff) {
            set.remove(&nums[lower]);
        }
    }

    false
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0220::contains_nearby_almost_duplicate;

    #[test]
    fn test_1() {
        assert!(contains_nearby_almost_duplicate(&[1, 2, 3, 1], 3, 0));
    }

    #[test]
    fn test_2() {
        assert!(!contains_nearby_almost_duplicate(&[1, 5, 9, 1, 5, 9], 2, 3));
    }
}
