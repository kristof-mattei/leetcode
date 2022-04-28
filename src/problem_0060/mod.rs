fn get_permutation(n: usize, k: usize) -> String {
    assert!((1..=9).contains(&n));
    assert!((1..=factorial(n)).contains(&k));

    let mut k = k - 1;

    let mut start_permutation = (1..=n as u8)
        .into_iter()
        .map(|v| (v + b'0'))
        .collect::<Vec<_>>();

    let mut permutation = String::new();
    for i in (0..n).rev() {
        let f = factorial(i);

        permutation.push(start_permutation.remove(k / f) as char);

        k %= f;
    }

    permutation
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

impl Solution {
    #[must_use]
    pub fn get_permutation(n: i32, k: i32) -> String {
        get_permutation(n as usize, k as usize)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0060::get_permutation;

    #[test]
    fn test_1() {
        assert_eq!(get_permutation(3, 3), "213");
    }

    #[test]
    fn test_2() {
        assert_eq!(get_permutation(4, 9), "2314");
    }

    #[test]
    fn test_3() {
        assert_eq!(get_permutation(3, 1), "123");
    }

    #[test]
    fn test_4() {
        assert_eq!(get_permutation(9, 100), "123495786");
    }
}
