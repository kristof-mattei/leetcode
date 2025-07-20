impl Solution {
    ///
    /// # Panics
    /// when nums is empty
    #[must_use]
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = std::collections::BinaryHeap::new();

        let max_len = nums.len() - (k as usize) + 1;

        for n in nums {
            h.push(n);

            if h.len() > max_len {
                h.pop();
            }
        }

        h.pop().unwrap()
    }
}

pub struct Solution;
