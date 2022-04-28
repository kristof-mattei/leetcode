use std::collections::HashSet;

fn four_sums(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    k_sum(&nums, target, 4)
}

fn k_sum(nums: &[i32], target: i32, k: usize) -> Vec<Vec<i32>> {
    // assert!(nums.is_sorted());

    let mut result = Vec::<Vec<i32>>::new();

    if nums.is_empty() {
        return result;
    }

    // ensure we can actually make a matching k with the values in
    // our nums
    let average_value = target / k as i32;

    // ensure average value falls between
    if nums[0] > average_value || average_value > nums[nums.len() - 1] {
        return result;
    }

    if k == 2 {
        return two_sum(nums, target);
    }

    for i in 0..nums.len() {
        // detect duplicates
        if i == 0 || nums[i - 1] != nums[i] {
            let subsets = k_sum(&nums[i + 1..], target - nums[i], k - 1);

            for mut subset in subsets {
                subset.push(nums[i]);

                result.push(subset);
            }
        }
    }

    result
}

/// nums needs to be sorted!
fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    // assert!(nums.is_sorted());

    let mut result = vec![];
    let mut hash_set = HashSet::new();

    for num in nums {
        if result.last().map_or(true, |v: &Vec<i32>| &v[0] != num)
            && hash_set.contains(&(target - num))
        {
            result.push(vec![*num, target - num]);
        }

        hash_set.insert(num);
    }

    result
}

impl Solution {
    #[must_use]
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        four_sums(nums, target)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0018::{four_sums, two_sum};

    #[test]
    fn test_2_sum() {
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 6), vec![vec![4, 2], vec![5, 1]]);
    }
    #[test]
    fn test() {
        assert_eq!(
            four_sums(vec![1, 0, -1, 0, -2, 2], 0),
            vec![[2, 1, -1, -2], [2, 0, 0, -2], [1, 0, 0, -1]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(four_sums(vec![2, 2, 2, 2, 2], 8), vec![[2, 2, 2, 2]]);
    }
}
