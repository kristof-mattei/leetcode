use std::cell::RefCell;
use std::rc::Rc;

use shared::TreeNode;

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        kth_smallest(root.as_ref(), k)
    }
}

fn kth_smallest_r(root: Option<&Rc<RefCell<TreeNode>>>, k: i32, rank: &mut i32) -> Option<i32> {
    let node = root?;

    let node = node.borrow();

    if let Some(val) = kth_smallest_r(node.left.as_ref(), k, rank) {
        return Some(val);
    }

    *rank += 1;

    if k == *rank {
        return Some(node.val);
    }

    if let Some(val) = kth_smallest_r(node.right.as_ref(), k, rank) {
        return Some(val);
    }

    None
}

fn kth_smallest(root: Option<&Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut rank = 0;

    kth_smallest_r(root, k, &mut rank).unwrap_or(0)
}

pub struct Solution;

#[cfg(test)]
mod tests {

    use shared::to_bt;

    use crate::problem_0230::kth_smallest;

    #[test]
    fn test_1() {
        let input: Vec<Option<i32>> = vec![3.into(), 1.into(), 4.into(), None, 2.into()];

        assert_eq!(kth_smallest(to_bt(&input).as_ref(), 1), 1);
    }

    #[test]
    fn test_2() {
        let input: Vec<Option<i32>> = vec![
            5.into(),
            3.into(),
            6.into(),
            2.into(),
            4.into(),
            None,
            None,
            1.into(),
        ];

        assert_eq!(kth_smallest(to_bt(&input).as_ref(), 3), 3);
    }
}
