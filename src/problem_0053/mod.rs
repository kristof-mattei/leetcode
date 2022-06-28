fn max_crossing_sum(nums: &[i32]) -> i32 {
    let middle = nums.len() / 2;

    let sum_left = nums
        .iter()
        .take(middle)
        .rev()
        .fold((0, i32::MIN), |(acc, max), &cur| {
            let sum = acc + cur;
            (sum, (sum).max(max))
        })
        .1;

    let sum_right = nums
        .iter()
        .skip(middle)
        .fold((0, i32::MIN), |(acc, max), &cur| {
            let sum = acc + cur;
            (sum, (sum).max(max))
        })
        .1;

    i32::max(sum_left + sum_right, i32::max(sum_left, sum_right))
}

fn max_sub_array(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let middle = nums.len() / 2;

    i32::max(
        max_crossing_sum(nums),
        i32::max(
            max_sub_array(&nums[0..middle]),
            max_sub_array(&nums[middle..]),
        ),
    )
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        max_sub_array(&nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0053::max_sub_array;

    #[test]
    fn test_1() {
        let input = [-2, 1, -3, 4, -1, 2, 1, -5, 4];

        assert_eq!(max_sub_array(&input), 6);
    }

    #[test]
    fn test_2() {
        let input = [1];

        assert_eq!(max_sub_array(&input), 1);
    }

    #[test]
    fn test_3() {
        let input = [5, 4, -1, 7, 8];

        assert_eq!(max_sub_array(&input), 23);
    }
}
