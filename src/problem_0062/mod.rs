use std::collections::HashMap;

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

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0062::unique_paths;

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
