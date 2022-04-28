fn dominant_index(nums: &[i32]) -> i32 {
    let mut is_largest = true;
    let mut largest = (0, 0);

    for (i, &num) in nums.iter().enumerate() {
        if num > largest.1 {
            is_largest = num >= largest.1 * 2;

            largest = (i, num);
        } else if num * 2 > largest.1 {
            is_largest = false;
        }
    }

    if is_largest {
        return largest.0 as i32;
    }

    -1
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        dominant_index(&nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0747::dominant_index;

    #[test]
    fn test_1() {
        let input = [3, 6, 1, 0];

        assert_eq!(dominant_index(&input), 1);
    }

    #[test]
    fn test_2() {
        let input = [1, 2, 3, 4];

        assert_eq!(dominant_index(&input), -1);
    }
    #[test]
    fn test_3() {
        let input = [1];

        assert_eq!(dominant_index(&input), 0);
    }
}
