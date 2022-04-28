use std::{cell::RefCell, rc::Rc};

use crate::shared::TreeNode;

fn sorted_array_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let mid = nums.len() / 2;

    Some(Rc::new(RefCell::new(TreeNode {
        val: nums[mid],
        left: sorted_array_to_bst(&nums[..mid]),
        right: sorted_array_to_bst(&nums[mid + 1..]),
    })))
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        sorted_array_to_bst(&nums)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::{problem_0108::sorted_array_to_bst, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            sorted_array_to_bst(&[-10, -3, 0, 5, 9]),
            to_bt(&[
                0.into(),
                (-3).into(),
                9.into(),
                (-10).into(),
                None,
                5.into()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            sorted_array_to_bst(&[-10, -3, 0, 5, 9]),
            to_bt(&[
                0.into(),
                (-3).into(),
                9.into(),
                (-10).into(),
                None,
                5.into()
            ])
        );
    }
}
