fn max_area(height: &[i32]) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut max_water = 0;

    while left < right {
        let left_height = height[left];
        let right_height = height[right];

        let current_water_capacity = i32::min(left_height, right_height) * (right - left) as i32;

        max_water = i32::max(max_water, current_water_capacity);

        if left_height < right_height {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_water
}

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn max_area(height: Vec<i32>) -> i32 {
        max_area(&height)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0011::max_area;

    #[test]
    fn test() {
        assert_eq!(max_area(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
    #[test]
    fn test_2() {
        assert_eq!(max_area(&[2, 3, 10, 5, 7, 8, 9]), 36);
    }
}
