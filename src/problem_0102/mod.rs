use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = vec![];

    let mut next_levels = vec![root];

    loop {
        let mut level_numbers = vec![];
        let mut children = vec![];

        for node in next_levels.iter().flatten() {
            let b = node.borrow();

            level_numbers.push(b.val);

            children.push(b.left.clone());
            children.push(b.right.clone());
        }

        if level_numbers.is_empty() {
            break;
        }

        next_levels = children;

        result.push(level_numbers);
    }

    result
}

impl Solution {
    #[must_use]
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        level_order(root)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0102::level_order, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            level_order(to_bt(&[
                3.into(),
                9.into(),
                20.into(),
                None,
                None,
                15.into(),
                7.into()
            ])),
            [vec![3], vec![9, 20], vec![15, 7]]
        );
    }
}
