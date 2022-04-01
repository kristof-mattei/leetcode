use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::{Solution, TreeNode};

fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(r) = root {
        let b = r.borrow();

        let mut answer = b.val;

        max_sub_path_sum(Some(r.clone()), &mut answer);

        answer
    } else {
        0
    }
}

fn max_sub_path_sum(node: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
    if let Some(n) = node {
        let b = n.borrow();

        let left_max = i32::max(0, max_sub_path_sum(b.left.clone(), max));
        let right_max = i32::max(0, max_sub_path_sum(b.right.clone(), max));

        *max = i32::max(*max, left_max + b.val + right_max);

        b.val + i32::max(left_max, right_max)
    } else {
        0
    }
}

impl Solution {
    #[must_use]
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_path_sum(root)
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_0124::max_path_sum, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(max_path_sum(to_bt(&[1.into(), 2.into(), 3.into()])), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            max_path_sum(to_bt(&[
                (-10).into(),
                9.into(),
                20.into(),
                None,
                None,
                15.into(),
                7.into()
            ])),
            42
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(max_path_sum(to_bt(&[1.into(), (-2).into(), 3.into()])), 4);
    }
}
