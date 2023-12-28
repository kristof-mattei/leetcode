use std::collections::HashMap;

fn memoize(
    cache: &mut HashMap<(usize, usize), usize>,
    matrix: &[Vec<i32>],
    lr: usize,
    lc: usize,
) -> usize {
    let key = (lr, lc);

    if cache.contains_key(&key) {
        return cache[&key];
    }

    let r = unique_paths_with_obstacles_r(cache, matrix, lr, lc);

    cache.insert(key, r);

    r
}

fn unique_paths_with_obstacles_r(
    cache: &mut HashMap<(usize, usize), usize>,
    matrix: &[Vec<i32>],
    lr: usize,
    lc: usize,
) -> usize {
    let max_row = matrix.len() - 1;
    let max_col = matrix.first().map_or(0, |r| r.len() - 1);

    if matrix.get(lr).and_then(|r| r.get(lc)).unwrap_or(&1) == &1 {
        return 0; // invalid
    }

    if lr == max_row && lc == max_col {
        return 1; // done
    }

    let mut result = 0;

    let possibility = memoize(cache, matrix, lr + 1, lc);

    result += possibility;

    let possibility = memoize(cache, matrix, lr, lc + 1);

    result += possibility;

    result
}

fn unique_paths_with_obstacles(obstacle_grid: &[Vec<i32>]) -> i32 {
    let r = unique_paths_with_obstacles_r(&mut HashMap::new(), obstacle_grid, 0, 0);

    r as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        unique_paths_with_obstacles(&obstacle_grid)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0063::unique_paths_with_obstacles;

    #[test]
    fn test_1() {
        assert_eq!(
            unique_paths_with_obstacles(&[vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(unique_paths_with_obstacles(&[vec![0, 1], vec![0, 0]]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(unique_paths_with_obstacles(&[vec![1]]), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(unique_paths_with_obstacles(&[vec![1, 0]]), 0);
    }
}
