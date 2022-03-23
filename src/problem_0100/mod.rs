use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::{Solution, TreeNode};
fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (None, Some(_)) | (Some(_), None) => false,
        (Some(l), Some(r)) => {
            let l_borrow = l.borrow();
            let r_borrow = r.borrow();
            l_borrow.val == r_borrow.val
                && is_same_tree(l_borrow.left.clone(), r_borrow.left.clone())
                && is_same_tree(l_borrow.right.clone(), r_borrow.right.clone())
        },
    }
}
impl Solution {
    #[must_use]
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        is_same_tree(p, q)
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_0100::is_same_tree, shared::to_bt};

    #[test]
    fn test_1() {
        assert!(is_same_tree(
            to_bt(&[1.into(), 2.into(), 3.into()]),
            to_bt(&[1.into(), 2.into(), 3.into()])
        ));
    }

    #[test]
    fn test_2() {
        assert!(!is_same_tree(
            to_bt(&[1.into(), 2.into()]),
            to_bt(&[1.into(), None, 2.into()])
        ));
    }

    #[test]
    fn test_3() {
        assert!(!is_same_tree(
            to_bt(&[1.into(), 2.into(), 1.into()]),
            to_bt(&[1.into(), 1.into(), 2.into()])
        ));
    }
}
