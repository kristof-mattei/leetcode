fn permute_unique_r(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();

    for i in 0..nums.len() {
        if i > 0 && nums[i - 1] == nums[i] {
            continue;
        }

        let v = nums.remove(i);

        for mut p in permute_unique_r(nums) {
            p.push(v);
            results.push(p);
        }

        nums.insert(i, v);
    }

    if results.is_empty() {
        return vec![vec![]];
    }

    results
}

fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    permute_unique_r(&mut nums)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        permute_unique(nums)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0047::permute_unique;

    #[test]
    fn test_1() {
        let input = [1, 2, 3];

        let mut result = permute_unique(input.to_vec());
        result.sort_unstable();

        assert_eq!(
            result,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    #[test]
    fn test_2() {
        let input = [1, 1, 2];

        let mut result = permute_unique(input.to_vec());
        result.sort_unstable();

        assert_eq!(result, vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]);
    }
}
