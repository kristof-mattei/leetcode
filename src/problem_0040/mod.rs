use crate::shared::Solution;

fn combination_sum_r(src: &[i32], target: i32) -> Vec<Vec<i32>> {
    let remaining_sum = src.iter().sum::<i32>();

    let mut results = Vec::new();

    if remaining_sum == target {
        results.push(src.to_vec());
        return results;
    }

    for (i, &v) in src.iter().enumerate() {
        // if the previous number is the same as ours
        // we don't need to use it
        if i > 0 && src[i - 1] == v {
            continue;
        }

        // can't get there
        if v > target {
            continue;
        }

        // match!
        if v == target {
            results.push(vec![v]);
            continue;
        }

        let combinations = combination_sum_r(&src[i + 1..], target - v);

        for mut combo in combinations {
            combo.push(v);

            results.push(combo);
        }
    }

    results
}

fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    combination_sum_r(&candidates, target)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        combination_sum2(candidates, target)
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_0040::combination_sum2, shared::sort_vec_of_vec};

    #[test]
    fn test_1() {
        let mut result = combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        sort_vec_of_vec(&mut result);

        assert_eq!(
            result,
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn test_2() {
        let mut result = combination_sum2(vec![2, 5, 2, 1, 2], 5);
        sort_vec_of_vec(&mut result);

        assert_eq!(result, vec![vec![1, 2, 2], vec![5]]);
    }

    #[test]
    fn test_3() {
        let mut result = combination_sum2(vec![4, 4, 2, 1, 4, 2, 2, 1, 3], 6);
        sort_vec_of_vec(&mut result);

        assert_eq!(
            result,
            vec![
                vec![1, 1, 2, 2],
                vec![1, 1, 4],
                vec![1, 2, 3],
                vec![2, 2, 2],
                vec![2, 4]
            ]
        );
    }

    #[test]
    fn test_4() {
        let mut result = combination_sum2(vec![4, 5, 3, 2, 3, 3, 5, 5], 9);
        sort_vec_of_vec(&mut result);

        assert_eq!(result, vec![vec![2, 3, 4], vec![3, 3, 3], vec![4, 5]]);
    }

    #[test]
    fn test_5() {
        let mut result = combination_sum2(
            vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
            30,
        );

        sort_vec_of_vec(&mut result);

        assert_eq!(
            result,
            vec![vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1
            ]]
        );
    }
}
