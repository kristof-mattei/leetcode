fn max_profit(prices: &[i32]) -> i32 {
    let mut max_profit = 0;

    for i in 1..prices.len() {
        if prices[i - 1] < prices[i] {
            max_profit += prices[i] - prices[i - 1];
        }
    }

    max_profit
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        max_profit(&prices)
    }
}

pub struct Solution {}

#[cfg(test)]
mod test {
    use crate::problem_0122::max_profit;

    #[test]
    fn test_1() {
        assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_profit(&[1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(max_profit(&[6, 1, 3, 2, 4, 7]), 7);
    }
}
