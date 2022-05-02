use std::{cell::RefCell, rc::Rc};

use crate::shared::TreeNode;

fn count_nodes(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    root.as_ref().map_or(0, |r| {
        let b = r.borrow();

        1 + count_nodes(&b.left) + count_nodes(&b.right)
    })
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        count_nodes(&root)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::shared::to_bt;

    use super::count_nodes;

    #[test]
    fn test_1() {
        assert_eq!(
            count_nodes(&to_bt(&[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6)
            ])),
            6
        );
    }
}
