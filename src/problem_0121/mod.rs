fn max_profit(prices: &[i32]) -> i32 {
    let mut buy_price = prices[0];
    let mut result = 0;

    for sell_price in prices.iter().skip(1) {
        if *sell_price < buy_price {
            buy_price = *sell_price;
        } else {
            result = i32::max(result, sell_price - buy_price);
        }
    }

    result
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
    use crate::problem_0121::max_profit;

    #[test]
    fn test_1() {
        assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(max_profit(&[2, 1, 4]), 3);
    }
}
