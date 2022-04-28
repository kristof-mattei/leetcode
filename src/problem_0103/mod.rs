use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = vec![];

    let mut next_levels = vec![root];

    let mut reverse = false;

    loop {
        let mut level_numbers = vec![];
        let mut children = vec![];

        for node in next_levels.iter().rev().flatten() {
            let b = node.borrow();

            level_numbers.push(b.val);
            if reverse {
                children.push(b.right.clone());
                children.push(b.left.clone());
            } else {
                children.push(b.left.clone());
                children.push(b.right.clone());
            }
        }

        if level_numbers.is_empty() {
            break;
        }

        reverse = !reverse;

        next_levels = children;

        result.push(level_numbers);
    }

    result
}

impl Solution {
    #[must_use]
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        zigzag_level_order(root)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::{problem_0103::zigzag_level_order, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            zigzag_level_order(to_bt(&[
                3.into(),
                0.into(),
                20.into(),
                None,
                None,
                15.into(),
                7.into()
            ])),
            [vec![3], vec![20, 0], vec![15, 7]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            zigzag_level_order(to_bt(&[
                1.into(),
                2.into(),
                3.into(),
                4.into(),
                None,
                None,
                5.into(),
            ])),
            [vec![1], vec![3, 2], vec![4, 5]]
        );
    }
}
