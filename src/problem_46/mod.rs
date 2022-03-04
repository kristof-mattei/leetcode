use crate::shared::Solution;

fn permute_r(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let len = nums.len();

    for i in 0..nums.len() {
        let v = nums[i];
        nums.swap_remove(i);

        for mut p in permute_r(nums) {
            p.push(v);
            results.push(p);
        }

        nums.push(v);
        nums.swap(i, len - 1);
    }

    if results.is_empty() {
        return vec![vec![]];
    }
    results
}

fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    permute_r(&mut nums)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        permute(nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_46::permute;

    #[test]
    fn test_1() {
        let input = [1, 2, 3];

        let mut result = permute(input.to_vec());
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
        let input = [1, 2, 3, 4];

        let mut result = permute(input.to_vec());
        result.sort_unstable();

        assert_eq!(
            result,
            vec![
                vec![1, 2, 3, 4],
                vec![1, 2, 4, 3],
                vec![1, 3, 2, 4],
                vec![1, 3, 4, 2],
                vec![1, 4, 2, 3],
                vec![1, 4, 3, 2],
                vec![2, 1, 3, 4],
                vec![2, 1, 4, 3],
                vec![2, 3, 1, 4],
                vec![2, 3, 4, 1],
                vec![2, 4, 1, 3],
                vec![2, 4, 3, 1],
                vec![3, 1, 2, 4],
                vec![3, 1, 4, 2],
                vec![3, 2, 1, 4],
                vec![3, 2, 4, 1],
                vec![3, 4, 1, 2],
                vec![3, 4, 2, 1],
                vec![4, 1, 2, 3],
                vec![4, 1, 3, 2],
                vec![4, 2, 1, 3],
                vec![4, 2, 3, 1],
                vec![4, 3, 1, 2],
                vec![4, 3, 2, 1]
            ]
        );
    }
}
