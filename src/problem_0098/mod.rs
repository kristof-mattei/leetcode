use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::{Solution, TreeNode};

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_r(root, None, None)
}

fn is_valid_bst_r(root: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
    root.map_or(true, |r| {
        let borrowed = r.borrow();
        let middle = borrowed.val;

        max.map_or(true, |m| middle < m)
            && min.map_or(true, |m| middle > m)
            && is_valid_bst_r(borrowed.left.clone(), min, Some(middle))
            && is_valid_bst_r(borrowed.right.clone(), Some(middle), max)
    })
}

impl Solution {
    #[must_use]
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_valid_bst(root)
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::to_bt;

    use super::is_valid_bst;

    #[test]
    fn test_1() {
        assert!(is_valid_bst(to_bt(&[2.into(), 1.into(), 3.into()])));
    }

    #[test]
    fn test_2() {
        assert!(!is_valid_bst(to_bt(&[
            5.into(),
            1.into(),
            4.into(),
            None,
            None,
            3.into(),
            6.into()
        ])));
    }

    #[test]
    fn test_3() {
        assert!(!is_valid_bst(to_bt(&[
            5.into(),
            4.into(),
            6.into(),
            None,
            None,
            3.into(),
            7.into()
        ])));
    }

    #[test]
    fn test_4() {
        assert!(is_valid_bst(to_bt(&[
            3.into(),
            1.into(),
            5.into(),
            0.into(),
            2.into(),
            4.into(),
            6.into()
        ])));
    }

    #[test]
    fn test_5() {
        assert!(is_valid_bst(to_bt(&[
            i32::MIN.into(),
            None,
            i32::MAX.into()
        ])));
    }
}
