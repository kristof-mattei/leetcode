use std::collections::VecDeque;

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value)]
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        max_sliding_window(&nums, k)
    }
}

fn max_sliding_window(nums: &[i32], k: i32) -> Vec<i32> {
    let window_size = k as usize;

    // window_indexes holds values in the current window from largest -> smallest
    let mut window_indexes = VecDeque::with_capacity(window_size);

    let mut maxes = Vec::with_capacity(nums.len() - window_size + 1);

    for (index, num) in nums.iter().enumerate() {
        // remove the indexes that we don't consider in our window anymore
        if let Some(&old_index) = window_indexes.front() {
            if index >= window_size && old_index <= index - window_size {
                window_indexes.pop_front();
            }
        }

        // remove all indexes with values smaller than current value
        while let Some(&old_index) = window_indexes.back() {
            if nums[old_index] < *num {
                window_indexes.pop_back();
            } else {
                break;
            }
        }

        window_indexes.push_back(index);

        // once we have collected our minimum window size, start pushing max to maxes
        if index >= (window_size - 1) {
            maxes.push(nums[*window_indexes.front().unwrap()]);
        }
    }

    maxes
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0239::max_sliding_window;

    #[test]
    fn test_1() {
        assert_eq!(
            max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            [3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(max_sliding_window(&[1], 1), [1]);
    }
}
