fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut dp: Vec<i32> = vec![0; n + 1];

    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=n {
        for j in 1..=i {
            dp[i] += dp[j - 1] * dp[i - j];
        }
    }

    dp[n]
}

impl Solution {
    #[must_use]
    pub fn num_trees(n: i32) -> i32 {
        num_trees(n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::num_trees;

    #[test]
    fn test_1() {
        assert_eq!(num_trees(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(num_trees(2), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(num_trees(3), 5);
    }

    #[test]
    fn test_4() {
        assert_eq!(num_trees(4), 14);
    }

    #[test]
    fn test_5() {
        assert_eq!(num_trees(5), 42);
    }

    #[test]
    fn test_6() {
        assert_eq!(num_trees(6), 132);
    }
}
