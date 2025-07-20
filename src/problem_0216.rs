fn backtrack(
    possibilities: &[i32],
    path: &mut Vec<i32>,
    k: usize,
    target: i32,
    result: &mut Vec<Vec<i32>>,
) {
    // requested length
    if path.len() == k {
        // and sum matches!?
        if target == 0 {
            // winner!
            result.push(path.clone());
        }

        // <LOUD ERROR SOUND>
        return;
    }

    // out of options
    if possibilities.is_empty() {
        return;
    }

    for i in 0..possibilities.len() {
        path.push(possibilities[i]);

        backtrack(
            &possibilities[i + 1..],
            path,
            k,
            target - possibilities[i],
            result,
        );

        path.pop();
    }
}

#[must_use]
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let possibilities: Vec<i32> = (1..=9).collect();

    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut path: Vec<i32> = Vec::new();

    backtrack(&possibilities, &mut path, k as usize, n, &mut result);

    result
}

impl Solution {
    #[must_use]
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        combination_sum3(k, n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0216::combination_sum3;

    #[test]
    fn test_1() {
        assert_eq!(combination_sum3(3, 7), [[1, 2, 4]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(combination_sum3(3, 9), [[1, 2, 6], [1, 3, 5], [2, 3, 4]]);
    }
}
