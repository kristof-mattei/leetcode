use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let mut results = vec![];
    let mut queue = Vec::from_iter([root]);

    while let Some(current) = queue.pop().flatten() {
        let left = current.borrow_mut().left.take();
        if left.is_some() {
            queue.push(Some(current));
            queue.push(left);
            continue;
        }

        results.push(current.borrow().val);

        let right = current.borrow_mut().right.take();

        if right.is_some() {
            queue.push(right);
        }
    }

    results
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        inorder_traversal(root)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    use crate::problem_0094::inorder_traversal;
    use crate::shared::to_bt;

    #[test]
    fn test_1() {
        let input = [1.into(), None, 2.into(), 3.into()];
        let t = to_bt(&input);

        let expected = [1, 3, 2];

        assert_eq!(inorder_traversal(t), expected);
    }

    #[test]
    fn test_2() {
        let input = [2.into(), 3.into(), None, 1.into()];
        let t = to_bt(&input);

        let expected = [1, 3, 2];

        assert_eq!(inorder_traversal(t), expected);
    }

    #[test]
    fn test_3() {
        let input = [3.into(), 1.into(), None, None, 2.into()];
        let t = to_bt(&input);

        let expected = [1, 2, 3];

        assert_eq!(inorder_traversal(t), expected);
    }

    #[test]
    fn test_4() {
        let input = [1.into(), 2.into(), None, None, 3.into()];
        let t = to_bt(&input);

        let expected = [2, 3, 1];

        assert_eq!(inorder_traversal(t), expected);
    }

    #[test]
    fn test_5() {
        let input = [2.into(), 1.into(), 3.into(), None, None, None, 4.into()];
        let t = to_bt(&input);

        let expected = [1, 2, 3, 4];

        assert_eq!(inorder_traversal(t), expected);
    }

    #[test]
    fn test_6() {
        let input = [4.into(), 1.into(), None, None, 3.into(), 2.into()];
        let t = to_bt(&input);

        let expected = [1, 2, 3, 4];

        assert_eq!(inorder_traversal(t), expected);
    }
}
