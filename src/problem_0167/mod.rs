use std::cmp::Ordering;

fn two_sum(numbers: &[i32], target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    loop {
        match (numbers[left] + numbers[right]).cmp(&target) {
            Ordering::Less => {
                left += 1;
            },
            Ordering::Equal => return vec![(left + 1) as i32, (right + 1) as i32],
            Ordering::Greater => {
                right -= 1;
            },
        }
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        two_sum(&numbers, target)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_1() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), [1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(two_sum(&[2, 3, 4], 6), [1, 3]);
    }
    #[test]
    fn test_3() {
        assert_eq!(two_sum(&[-1, 0], -1), [1, 2]);
    }
}
