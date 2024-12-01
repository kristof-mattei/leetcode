impl Solution {
    #[expect(clippy::ptr_arg)]
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        wiggle_sort(nums);
    }
}

fn wiggle_sort(_nums: &mut [i32]) {
    // later
}

pub struct Solution;
