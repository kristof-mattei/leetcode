use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut first_node: Option<Rc<RefCell<TreeNode>>> = None;
    let mut second_node: Option<Rc<RefCell<TreeNode>>> = None;
    let mut previous_node: Option<Rc<RefCell<TreeNode>>> = None;

    let mut current_node = root.clone();

    while current_node.is_some() {
        let left_node = current_node.as_ref().unwrap().borrow().left.clone();

        if let Some(mut n) = left_node {
            loop {
                let right_node = n.borrow().right.clone();

                if let Some(right) = right_node {
                    if &right != current_node.as_ref().unwrap() {
                        n = right;
                        continue;
                    }
                }

                break;
            }

            let right_node = n.borrow().right.clone();

            if right_node.is_none() {
                n.borrow_mut().right = current_node.clone();
                current_node = current_node.unwrap().borrow().left.clone();
                continue;
            }

            n.borrow_mut().right = None;
        }

        // test if we have mismatch
        if previous_node.as_ref().map_or(false, |p| {
            current_node.as_ref().unwrap().borrow().val < p.borrow().val
        }) {
            // if it's the first mismatch, store in first_node
            if first_node.is_none() {
                first_node = previous_node;
            }
            // always update second node
            second_node = current_node.clone();
        }
        previous_node = current_node.clone();
        current_node = current_node.unwrap().borrow().right.clone();
    }

    std::mem::swap(
        &mut first_node.as_ref().unwrap().borrow_mut().val,
        &mut second_node.as_ref().unwrap().borrow_mut().val,
    );
}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        recover_tree(root);
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0099::recover_tree, shared::to_bt};

    #[test]
    fn test_1() {
        let mut tree = to_bt(&[1.into(), 3.into(), None, None, 2.into()]);
        recover_tree(&mut tree);
        assert_eq!(tree, to_bt(&[3.into(), 1.into(), None, None, 2.into()]));
    }
}
