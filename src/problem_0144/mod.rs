use std::{cell::RefCell, rc::Rc};

use crate::shared::TreeNode;

fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut results = vec![];
    if let Some(r) = root {
        let mut left = vec![r];
        let mut right = Vec::new();

        while let Some(node) = left.pop().or_else(|| right.pop()) {
            let mut borrow = node.borrow_mut();

            results.push(borrow.val);

            if let Some(l) = borrow.left.take() {
                left.push(l);
            }

            if let Some(r) = borrow.right.take() {
                right.push(r);
            }
        }
    }
    results
}

impl Solution {
    #[must_use]
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        preorder_traversal(root)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::{problem_0144::preorder_traversal, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            preorder_traversal(to_bt(&[1.into(), None, 2.into(), 3.into()])),
            [1, 2, 3]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(preorder_traversal(to_bt(&[])), []);
    }

    #[test]
    fn test_3() {
        assert_eq!(preorder_traversal(to_bt(&[1.into()])), [1]);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            preorder_traversal(to_bt(&[1.into(), 4.into(), 3.into(), 2.into()])),
            [1, 4, 2, 3]
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            preorder_traversal(to_bt(&[2.into(), 1.into(), 3.into(), None, 4.into()])),
            [2, 1, 4, 3]
        );
    }
}
