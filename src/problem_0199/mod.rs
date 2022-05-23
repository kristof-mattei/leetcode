use std::{cell::RefCell, rc::Rc};

fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut history = vec![];

    right_side_view_r(root, 0, &mut history);

    history
}

fn right_side_view_r(
    node: Option<Rc<RefCell<TreeNode>>>,
    current_depth: usize,
    history: &mut Vec<i32>,
) {
    if let Some(c) = node {
        let mut b = c.borrow_mut();

        if history.len() <= current_depth {
            history.push(b.val);
        }

        right_side_view_r(b.right.take(), current_depth + 1, history);
        right_side_view_r(b.left.take(), current_depth + 1, history);
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        right_side_view(root)
    }
}

pub struct Solution;

use crate::shared::TreeNode;

#[cfg(test)]
mod tests {
    use crate::{problem_0199::right_side_view, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            right_side_view(to_bt(&[
                Some(1),
                Some(2),
                Some(3),
                None,
                Some(5),
                None,
                Some(4)
            ])),
            &[1, 3, 4]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(right_side_view(to_bt(&[Some(1), None, Some(3)])), &[1, 3]);
    }

    #[test]
    fn test_3() {
        assert_eq!(right_side_view(to_bt(&[])), &[]);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            right_side_view(to_bt(&[Some(1), Some(2), Some(3), Some(4)])),
            &[1, 3, 4]
        );
    }
}
