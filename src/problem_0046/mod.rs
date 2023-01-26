fn permute_r(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let len = nums.len();

    for i in 0..len {
        let v = nums.remove(i);

        for mut p in permute_r(nums) {
            let mut n = vec![v];
            n.append(&mut p);
            results.push(n);
        }

        nums.insert(i, v);
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

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0046::permute;

    #[test]
    fn test_1() {
        let input = [1, 2, 3];

        let result = permute(input.to_vec());

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

        let result = permute(input.to_vec());

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
