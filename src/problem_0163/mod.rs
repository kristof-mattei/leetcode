use crate::shared::Solution;

fn maximum_gap(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();

    let mut max = 0;

    for w in nums.windows(2) {
        let left = w[0] as u32;
        let right = w[1] as u32;

        let diff = if left < right {
            right.wrapping_sub(left)
        } else {
            left.wrapping_sub(right)
        };

        max = u32::max(max, diff);
    }

    max as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        maximum_gap(nums)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0163::maximum_gap;

    #[test]
    fn test_1() {
        assert_eq!(maximum_gap(vec![3, 6, 9, 1]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(maximum_gap(vec![10]), 0);
    }
}
