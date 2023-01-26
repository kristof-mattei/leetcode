fn max_profit(prices: &[i32]) -> i32 {
    // buy Price at First Day
    let mut buy = 0 - prices[0];
    // max Overall Profit till date
    let mut max_overal_profit = 0;

    // max profit if not sold on a give date
    let mut max_overal_profit_not_sold = 0;

    #[allow(clippy::needless_range_loop)]
    for i in 1..prices.len() {
        // Total profit if bought today;
        let current = max_overal_profit_not_sold - prices[i];

        // max Profit if not sold today
        max_overal_profit_not_sold = i32::max(max_overal_profit_not_sold, max_overal_profit);
        // max Profit if selling today was an option
        max_overal_profit = i32::max(max_overal_profit, buy + prices[i]);
        // best buying position;
        buy = i32::max(current, buy);
    }

    max_overal_profit
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        max_profit(&prices)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0309::max_profit;

    #[test]
    fn test_1() {
        assert_eq!(max_profit(&[1, 2, 3, 0, 2]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_profit(&[1]), 0);
    }
}
