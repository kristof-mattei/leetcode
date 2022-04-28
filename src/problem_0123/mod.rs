fn max_profit(prices: &[i32]) -> i32 {
    let mut buy_1 = i32::MAX;
    let mut buy_2 = i32::MAX;

    let mut profit_1 = 0;
    let mut profit_2 = 0;

    for &price in prices {
        buy_1 = buy_1.min(price);
        profit_1 = profit_1.max(price - buy_1);

        buy_2 = buy_2.min(price - profit_1);
        profit_2 = profit_2.max(price - buy_2);
    }
    profit_2
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
mod test {
    use crate::problem_0123::max_profit;

    #[test]
    fn test_1() {
        assert_eq!(max_profit(&[3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_profit(&[1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
    }

    // #[test]
    // fn test_4() {
    //     assert_eq!(max_profit(&[6, 1, 3, 2, 4, 7]), 7);
    // }
}
