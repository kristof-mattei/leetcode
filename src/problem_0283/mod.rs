impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::ptr_arg)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        move_zeroes(nums);
    }
}

fn move_zeroes(nums: &mut [i32]) {
    let mut last_zero_index = 0;

    for non_zero_index in 0..nums.len() {
        if nums[non_zero_index] != 0 {
            nums.swap(non_zero_index, last_zero_index);
            last_zero_index += 1;
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0283::move_zeroes;

    #[test]
    fn test_1() {
        let mut result = vec![0, 1, 0, 3, 12];

        move_zeroes(&mut result);

        assert_eq!(result, &[1, 3, 12, 0, 0]);
    }
}
