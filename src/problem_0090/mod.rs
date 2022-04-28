use std::collections::HashSet;

fn subsets_with_dup_r(nums: &[i32]) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![vec![]];
    }

    let mut results = Vec::new();

    for i in 0..nums.len() {
        let v = nums[i];

        results.push(vec![v]);

        for mut p in subsets_with_dup_r(&nums[i + 1..]) {
            let mut n = vec![v];
            n.append(&mut p);
            results.push(n);
        }
    }

    results
}

fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();

    let mut h = [vec![]].into_iter().collect::<HashSet<Vec<i32>>>();

    for subset in subsets_with_dup_r(&nums) {
        h.insert(subset);
    }

    h.into_iter().collect::<Vec<_>>()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        subsets_with_dup(nums)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0090::subsets_with_dup;

    #[test]
    fn test_1() {
        let input = vec![1, 2, 2];

        let mut expected = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];

        expected.sort_unstable();

        let mut result = subsets_with_dup(input);
        result.sort_unstable();

        assert_eq!(expected, result);
    }
}
