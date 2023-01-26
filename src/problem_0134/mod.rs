fn can_complete_circuit(gas: &[i32], cost: &[i32]) -> i32 {
    if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() {
        return -1;
    }

    let mut total = 0;
    let mut start = 0;
    for i in 0..gas.len() {
        total += gas[i] - cost[i];
        if total < 0 {
            start = i + 1;
            total = 0;
        }
    }

    start as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        can_complete_circuit(&gas, &cost)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0134::can_complete_circuit;

    #[test]
    fn test_1() {
        assert_eq!(can_complete_circuit(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(can_complete_circuit(&[2, 3, 4], &[3, 4, 3]), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(can_complete_circuit(&[5, 1, 2, 3, 4], &[4, 4, 1, 5, 1]), 4);
    }

    #[test]
    fn test_4() {
        assert_eq!(can_complete_circuit(&[5, 8, 2, 8], &[6, 5, 6, 6]), 3);
    }
}
