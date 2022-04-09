use std::{cell::RefCell, rc::Rc};

use crate::shared::{Solution, TreeNode};

fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if let Some(r) = root {
        let b = r.borrow();

        if b.left.is_none() && b.right.is_none() {
            target_sum == b.val
        } else {
            has_path_sum(b.left.clone(), target_sum - b.val)
                || has_path_sum(b.right.clone(), target_sum - b.val)
        }
    } else {
        false
    }
}

impl Solution {
    #[must_use]
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        has_path_sum(root, target_sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_0112::has_path_sum, shared::to_bt};

    #[test]
    fn test_1() {
        assert!(has_path_sum(
            to_bt(&[
                5.into(),
                4.into(),
                8.into(),
                11.into(),
                None,
                13.into(),
                4.into(),
                7.into(),
                2.into(),
                None,
                None,
                None,
                1.into()
            ]),
            22
        ));
    }

    #[test]
    fn test_2() {
        assert!(!has_path_sum(to_bt(&[1.into(), 2.into(), 3.into(),]), 5));
    }

    #[test]
    fn test_3() {
        assert!(!has_path_sum(to_bt(&[]), 0));
    }
}
