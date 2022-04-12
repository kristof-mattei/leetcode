use std::cmp;

use crate::shared::Solution;

fn candy(ratings: &[i32]) -> i32 {
    let len = ratings.len();

    let mut counts = vec![1; len];

    for i in 1..len {
        if ratings[i] > ratings[i - 1] {
            counts[i] = counts[i - 1] + 1;
        }
    }

    let mut sum: i32 = counts[len - 1];

    for i in (0..len - 1).rev() {
        counts[i] = if ratings[i] > ratings[i + 1] {
            cmp::max(counts[i + 1] + 1, counts[i])
        } else {
            counts[i]
        };

        sum += counts[i];
    }

    sum
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn candy(ratings: Vec<i32>) -> i32 {
        candy(&ratings)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0135::candy;

    #[test]
    fn test_1() {
        assert_eq!(candy(&[1, 0, 2]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(candy(&[1, 2, 2]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(candy(&[1, 2, 87, 87, 87, 2, 1]), 13);
    }

    #[test]
    fn test_4() {
        assert_eq!(candy(&[0]), 1);
    }
}
