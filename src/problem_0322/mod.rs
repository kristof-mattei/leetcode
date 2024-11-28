use std::collections::HashMap;

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        coin_change(&coins, amount)
    }
}

fn coin_change(coins: &[i32], amount: i32) -> i32 {
    let mut cache = HashMap::new();

    let result = dynamic(amount, coins, &mut cache);

    result.unwrap_or(-1)
}

fn dynamic(amount: i32, coins: &[i32], cache: &mut HashMap<i32, Option<i32>>) -> Option<i32> {
    if let Some(&result) = cache.get(&amount) {
        return result;
    }

    if amount == 0 {
        return Some(0);
    }

    let mut answer = None;

    for &coin in coins {
        if coin <= amount {
            // the 1 is the current coin
            let sub_result = dynamic(amount - coin, coins, cache).map(|s_r| s_r + 1);

            if sub_result.is_some() {
                if answer.is_some() {
                    answer = answer.min(sub_result);
                } else {
                    answer = sub_result;
                }
            }
        }
    }

    cache.insert(amount, answer);

    answer
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0322::coin_change;

    #[test]
    fn test_1() {
        assert_eq!(coin_change(&[1, 2, 5], 11), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(coin_change(&[2], 3), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(coin_change(&[1], 0), 0);
    }
}
