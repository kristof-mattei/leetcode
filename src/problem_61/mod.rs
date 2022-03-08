use std::collections::HashMap;

use crate::shared::Solution;

// fn unique_paths_r(cache: &mut Vec<Vec<usize>>, m: usize, n: usize) -> usize {
//     if m == 0 || n == 0 {
//         return 1;
//     }

//     if cache[n][m] > 0 {
//         return cache[n][m];
//     }

//     let next_m = if m > 0 {
//         unique_paths_r(cache, m - 1, n)
//     } else {
//         0
//     };

//     let next_n = if n > 0 {
//         unique_paths_r(cache, m, n - 1)
//     } else {
//         0
//     };

//     cache[n][m] = next_m + next_n;
//     cache[n][m]
// }

fn memoize(
    cache: &mut HashMap<(usize, usize), usize>,
    m: usize,
    n: usize,
    lr: usize,
    lc: usize,
) -> usize {
    let key = (lr, lc);

    if cache.contains_key(&key) {
        return cache[&key];
    }

    let r = unique_paths_r(cache, m, n, lr, lc);

    cache.insert(key, r);

    r
}

fn unique_paths_r(
    cache: &mut HashMap<(usize, usize), usize>,
    m: usize,
    n: usize,
    lr: usize,
    lc: usize,
) -> usize {
    if lr == m && lc == n {
        return 1; // done
    }

    let mut result = 0;

    if lr < m {
        let possibility = memoize(cache, m, n, lr + 1, lc);

        result += possibility;
    }

    if lc < n {
        let possibility = memoize(cache, m, n, lr, lc + 1);

        result += possibility;
    }

    result
}
fn unique_paths(m: usize, n: usize) -> i32 {
    let r = unique_paths_r(&mut HashMap::new(), m - 1, n - 1, 0, 0);

    r as i32
}

impl Solution {
    #[must_use]
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        unique_paths(m as usize, n as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_61::unique_paths;

    #[test]
    fn test_1() {
        assert_eq!(unique_paths(3, 2), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn test_3() {
        assert_eq!(unique_paths(23, 12), 193_536_720);
    }
}
