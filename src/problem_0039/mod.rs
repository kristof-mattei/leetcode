fn combination_sum_r(src: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut results = Vec::new();

    for (i, &v) in src.iter().enumerate() {
        if v > target {
            continue;
        }

        if v == target {
            results.push(vec![v]);
            continue;
        }

        let x = combination_sum_r(&src[i..], target - v);

        for mut y in x {
            y.push(v);
            results.push(y);
        }
    }

    results
}

fn combination_sum(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
    combination_sum_r(candidates, target)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        combination_sum(&candidates, target)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0039::combination_sum;
    use crate::shared::sort_vec_of_vec;

    #[test]
    fn test_1() {
        let mut result = combination_sum(&[2, 3, 6, 7], 7);
        sort_vec_of_vec(&mut result);

        assert_eq!(result, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn test_2() {
        let mut result = combination_sum(&[2, 3, 5], 8);
        sort_vec_of_vec(&mut result);

        assert_eq!(result, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }

    #[test]
    fn test_3() {
        let mut result = combination_sum(&[2], 1);
        sort_vec_of_vec(&mut result);

        assert_eq!(result, Vec::<Vec::<i32>>::new());
    }

    #[test]
    fn test_4() {
        let mut result = combination_sum(&[1, 2], 4);
        sort_vec_of_vec(&mut result);

        assert_eq!(result, vec![vec![1, 1, 1, 1], vec![1, 1, 2], vec![2, 2]]);
    }

    #[test]
    fn test_5() {
        let mut result = combination_sum(&[2, 7, 6, 3, 5, 1], 9);
        sort_vec_of_vec(&mut result);

        assert_eq!(
            result,
            vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 2],
                vec![1, 1, 1, 1, 1, 1, 3],
                vec![1, 1, 1, 1, 1, 2, 2],
                vec![1, 1, 1, 1, 2, 3],
                vec![1, 1, 1, 1, 5],
                vec![1, 1, 1, 2, 2, 2],
                vec![1, 1, 1, 3, 3],
                vec![1, 1, 1, 6],
                vec![1, 1, 2, 2, 3],
                vec![1, 1, 2, 5],
                vec![1, 1, 7],
                vec![1, 2, 2, 2, 2],
                vec![1, 2, 3, 3],
                vec![1, 2, 6],
                vec![1, 3, 5],
                vec![2, 2, 2, 3],
                vec![2, 2, 5],
                vec![2, 7],
                vec![3, 3, 3],
                vec![3, 6]
            ]
        );
    }

    #[test]
    fn test_6() {
        let mut result = combination_sum(&[3, 12, 9, 11, 6, 7, 8, 5, 4], 15);
        sort_vec_of_vec(&mut result);

        assert_eq!(
            result,
            vec![
                vec![3, 3, 3, 3, 3],
                vec![3, 3, 3, 6],
                vec![3, 3, 4, 5],
                vec![3, 3, 9],
                vec![3, 4, 4, 4],
                vec![3, 4, 8],
                vec![3, 5, 7],
                vec![3, 6, 6],
                vec![3, 12],
                vec![4, 4, 7],
                vec![4, 5, 6],
                vec![4, 11],
                vec![5, 5, 5],
                vec![6, 9],
                vec![7, 8]
            ]
        );
    }
}
