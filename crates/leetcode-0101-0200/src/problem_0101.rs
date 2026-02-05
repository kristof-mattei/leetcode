use std::cell::RefCell;
use std::rc::Rc;

use shared::TreeNode;

fn is_symmetric_r(
    left: Option<&Rc<RefCell<TreeNode>>>,
    right: Option<&Rc<RefCell<TreeNode>>>,
) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(l), Some(r)) => {
            let l_borrow = l.borrow();
            let r_borrow = r.borrow();

            if l_borrow.val != r_borrow.val {
                return false;
            }

            if is_symmetric_r(l_borrow.left.as_ref(), r_borrow.right.as_ref()) {
                return is_symmetric_r(l_borrow.right.as_ref(), r_borrow.left.as_ref());
            }

            false
        },
        _ => false,
    }
}

fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(r) = root {
        let borrow = r.borrow();

        return is_symmetric_r(borrow.left.as_ref(), borrow.right.as_ref());
    }

    true
}

impl Solution {
    #[must_use]
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_symmetric(root)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use shared::to_bt;

    use crate::problem_0101::is_symmetric;

    #[test]
    fn test_1() {
        assert!(is_symmetric(to_bt(&[
            1.into(),
            2.into(),
            2.into(),
            3.into(),
            4.into(),
            4.into(),
            3.into()
        ])));
    }

    #[test]
    fn test_2() {
        assert!(!is_symmetric(to_bt(&[
            1.into(),
            2.into(),
            2.into(),
            None,
            3.into(),
            None,
            3.into()
        ])));
    }
}
