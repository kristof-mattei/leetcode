use crate::shared::Solution;

fn combine_r(numbers: &[i32], k: i32) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![vec![]];
    }

    let mut results = Vec::new();

    for (i, &n) in numbers.iter().enumerate() {
        for mut c in combine_r(&numbers[i + 1..], k - 1) {
            c.push(n);
            results.push(c);
        }
    }

    results
}

fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let numbers = (1..=n).collect::<Vec<_>>();

    combine_r(&numbers, k)
}

impl Solution {
    #[must_use]
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        combine(n, k)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0077::combine;

    #[test]
    fn test_1() {
        assert_eq!(
            combine(4, 2),
            [[2, 1], [3, 1], [4, 1], [3, 2], [4, 2], [4, 3]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(combine(1, 1), [[1]]);
    }
}
