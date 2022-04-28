use std::{cell::RefCell, rc::Rc};

use crate::shared::TreeNode;

fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut results = vec![];
    if let Some(r) = root {
        let mut queue = vec![r];

        while let Some(node) = queue.pop() {
            let mut borrow = node.borrow_mut();

            match (borrow.left.take(), borrow.right.take()) {
                (None, None) => {
                    results.push(borrow.val);
                },
                (Some(l), None) => {
                    queue.push(node.clone());
                    queue.push(l);
                },
                (None, Some(r)) => {
                    queue.push(node.clone());
                    queue.push(r);
                },
                (Some(l), Some(r)) => {
                    queue.push(node.clone());
                    queue.push(r);

                    queue.push(l);
                },
            }
        }
    }
    results
}

impl Solution {
    #[must_use]
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        postorder_traversal(root)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::{problem_0145::postorder_traversal, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            postorder_traversal(to_bt(&[1.into(), None, 2.into(), 3.into()])),
            [3, 2, 1]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(postorder_traversal(to_bt(&[])), []);
    }

    #[test]
    fn test_3() {
        assert_eq!(postorder_traversal(to_bt(&[1.into()])), [1]);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            postorder_traversal(to_bt(&[3.into(), 1.into(), 2.into()])),
            [1, 2, 3]
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            postorder_traversal(to_bt(&[3.into(), 1.into(), None, None, 2.into()])),
            [2, 1, 3]
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            postorder_traversal(to_bt(&[4.into(), 2.into(), None, 1.into(), 3.into()])),
            [1, 3, 2, 4]
        );
    }
}
