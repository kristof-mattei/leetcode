use crate::shared::Solution;

fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in (0..digits.len()).rev() {
        digits[i] += 1;

        if digits[i] == 10 {
            digits[i] = 0;
        } else {
            return digits;
        }
    }

    digits.insert(0, 1);

    digits
}

impl Solution {
    #[must_use]
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        plus_one(digits)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0066::plus_one;

    #[test]
    fn test_1() {
        assert_eq!(plus_one(vec![1, 2, 3]), &[1, 2, 4]);
    }
    #[test]
    fn test_2() {
        assert_eq!(plus_one(vec![4, 3, 2, 1]), &[4, 3, 2, 2]);
    }
    #[test]
    fn test_3() {
        assert_eq!(plus_one(vec![9]), &[1, 0]);
    }
}
