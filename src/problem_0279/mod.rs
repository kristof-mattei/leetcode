use std::cmp::Ordering;
use std::collections::VecDeque;

fn num_squares(n: i32) -> i32 {
    let mut dp = vec![0; n as usize];

    let mut queue = VecDeque::from(vec![(0, 0)]);

    while !queue.is_empty() {
        let (val, step) = queue.pop_front().unwrap();

        for i in 1..100 {
            let target = val + i * i;

            match target.cmp(&n) {
                Ordering::Less => {
                    if dp[target as usize] == 0 {
                        dp[target as usize] = step + 1;
                        queue.push_back((target, step + 1));
                    }
                },
                Ordering::Equal => {
                    return step + 1;
                },
                Ordering::Greater => {
                    break;
                },
            }
        }
    }

    -1
}

impl Solution {
    #[must_use]
    pub fn num_squares(n: i32) -> i32 {
        num_squares(n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0279::num_squares;

    #[test]
    fn test_1() {
        assert_eq!(num_squares(12), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(num_squares(13), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(num_squares(1), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(num_squares(45), 2);
    }

    #[test]
    fn test_5() {
        assert_eq!(num_squares(48), 3);
    }
}
