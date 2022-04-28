use std::{cell::RefCell, rc::Rc};

use crate::shared::{ListNode, TreeNode};

fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut arr = vec![];

    while let Some(h) = head {
        arr.push(h.val);

        head = h.next;
    }

    sorted_list_to_bst_r(&arr)
}

fn sorted_list_to_bst_r(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let mid = nums.len() / 2;

    Some(Rc::new(RefCell::new(TreeNode {
        val: nums[mid],
        left: sorted_list_to_bst_r(&nums[..mid]),
        right: sorted_list_to_bst_r(&nums[mid + 1..]),
    })))
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        sorted_list_to_bst(head)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{
        problem_0109::sorted_list_to_bst,
        shared::{to_bt, to_ll},
    };

    #[test]
    fn test_1() {
        assert_eq!(
            sorted_list_to_bst(to_ll(&[-10, -3, 0, 5, 9])),
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
            sorted_list_to_bst(to_ll(&[-10, -3, 0, 5, 9])),
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
