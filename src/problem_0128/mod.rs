use std::collections::BTreeSet;

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set = BTreeSet::<i32>::from_iter(nums);

    set.iter()
        .filter_map(|n| {
            if set.contains(&(n - 1)) {
                None
            } else {
                let mut current = n + 1;

                while set.contains(&current) {
                    current += 1;
                }

                Some(current - n)
            }
        })
        .max()
        .unwrap_or(0)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        longest_consecutive(nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0128::longest_consecutive;

    #[test]
    fn test_1() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
