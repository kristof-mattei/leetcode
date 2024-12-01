use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

fn count_nodes(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    root.as_ref().map_or(0, |r| {
        let b = r.borrow();

        1 + count_nodes(b.left.as_ref()) + count_nodes(b.right.as_ref())
    })
}

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value)]
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        count_nodes(root.as_ref())
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::count_nodes;
    use crate::shared::to_bt;

    #[test]
    fn test_1() {
        assert_eq!(
            count_nodes(to_bt(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]).as_ref()),
            6
        );
    }
}
