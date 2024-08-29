use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        kth_smallest(&root, k)
    }
}

fn kth_smallest_r(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, rank: &mut i32) -> Option<i32> {
    let Some(node) = root else {
        return None;
    };

    let node = node.borrow();

    if let Some(val) = kth_smallest_r(&node.left, k, rank) {
        return Some(val);
    }

    *rank += 1;

    if k == *rank {
        return Some(node.val);
    }

    if let Some(val) = kth_smallest_r(&node.right, k, rank) {
        return Some(val);
    }

    None
}

fn kth_smallest(root: &Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut rank = 0;

    kth_smallest_r(root, k, &mut rank).unwrap_or(0)
}

pub struct Solution;

#[cfg(test)]
mod tests {

    use crate::problem_0230::kth_smallest;
    use crate::shared::to_bt;

    #[test]
    fn test_1() {
        let input: Vec<Option<i32>> = vec![3.into(), 1.into(), 4.into(), None, 2.into()];

        assert_eq!(kth_smallest(&to_bt(&input), 1), 1);
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

        assert_eq!(kth_smallest(&to_bt(&input), 3), 3);
    }
}
