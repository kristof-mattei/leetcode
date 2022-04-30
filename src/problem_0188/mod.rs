fn max_profit(k: i32, prices: &[i32]) -> i32 {
    let transactions = k as usize;

    let mut dp: Vec<Vec<i32>> = vec![vec![0; prices.len() + 1]; transactions + 1];

    for t in 1..=transactions {
        let mut min_buy = i32::MAX;

        for day in 1..=prices.len() {
            min_buy = i32::min(min_buy, prices[day - 1] - dp[t - 1][day]);

            dp[t][day] = i32::max(dp[t][day - 1], prices[day - 1] - min_buy);
        }
    }

    dp[k as usize][prices.len()]
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        max_profit(k, &prices)
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use crate::problem_0188::max_profit;

    #[test]
    fn test_1() {
        assert_eq!(max_profit(2, &[2, 4, 1]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_profit(2, &[3, 2, 6, 5, 0, 3]), 7);
    }
}
