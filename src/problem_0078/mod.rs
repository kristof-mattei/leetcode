fn subsets_r(nums: &[i32], k: usize) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![vec![]];
    }

    let mut results = Vec::new();

    let len = nums.len();

    for i in 0..len {
        let v = nums[i];

        for mut p in subsets_r(&nums[i + 1..], k - 1) {
            let mut n = vec![v];
            n.append(&mut p);
            results.push(n);
        }
    }

    results
}

fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut results = Vec::new();

    for k in 0..=nums.len() {
        results.append(&mut subsets_r(nums, k));
    }

    results
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        subsets(&nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0078::subsets;

    #[test]
    fn test_1() {
        assert_eq!(
            subsets(&[1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(subsets(&[0]), vec![vec![], vec![0],]);
    }
}
