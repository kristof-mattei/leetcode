use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

fn max_depth_r(root: Option<Rc<RefCell<TreeNode>>>, depth: usize) -> usize {
    if let Some(r) = root {
        let b = r.borrow();

        usize::max(
            max_depth_r(b.left.clone(), depth + 1),
            max_depth_r(b.right.clone(), depth + 1),
        )
    } else {
        depth
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
    max_depth_r(root, 0)
}

impl Solution {
    #[must_use]
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_depth(root) as i32
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::{problem_0104::max_depth, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            max_depth(to_bt(&[
                3.into(),
                9.into(),
                20.into(),
                None,
                None,
                15.into(),
                7.into()
            ])),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(max_depth(to_bt(&[1.into(), None, 2.into(),])), 2);
    }
}
