use std::{cell::RefCell, rc::Rc};

use crate::shared::TreeNode;

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        invert_tree(root)
    }
}

fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(root) = root.as_mut() {
        let mut root_borrow = root.borrow_mut();

        let left = root_borrow.left.take();
        let right = root_borrow.right.take();

        root_borrow.left = invert_tree(right);
        root_borrow.right = invert_tree(left);
    }
    root
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0226::invert_tree, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            invert_tree(to_bt(
                &[4, 2, 7, 1, 3, 6, 9]
                    .into_iter()
                    .map(Option::Some)
                    .collect::<Vec<Option<_>>>()
            )),
            to_bt(
                &[4, 7, 2, 9, 6, 3, 1]
                    .into_iter()
                    .map(Option::Some)
                    .collect::<Vec<Option<_>>>()
            )
        );
    }
}
