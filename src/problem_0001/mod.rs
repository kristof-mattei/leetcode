use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Vec<i32> {
    let mut hash_set = HashMap::<i32, usize>::new();

    for (i, &num) in nums.iter().enumerate() {
        if let Some(&n) = hash_set.get(&(target - num)) {
            return vec![n as i32, i as i32];
        }

        hash_set.insert(num, i);
    }

    unreachable!()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        two_sum(&nums, target)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0001::two_sum;

    #[test]
    fn test_1() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), [0, 1]);
    }
    #[test]

    fn test_2() {
        assert_eq!(two_sum(&[3, 2, 4], 6), [1, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(two_sum(&[3, 3], 6), [0, 1]);
    }
}
