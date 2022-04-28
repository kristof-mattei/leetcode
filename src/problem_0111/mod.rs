use crate::shared::TreeNode;
use std::cell::RefCell;
use std::{collections::VecDeque, rc::Rc};

fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut queue = VecDeque::from_iter([(root, 0)]);

    while let Some((node, depth)) = queue.pop_front() {
        if let Some(n) = node {
            let b = n.borrow();

            let left = b.left.clone();
            let right = b.right.clone();

            if left.is_none() && right.is_none() {
                return depth + 1;
            }

            queue.push_back((left, depth + 1));
            queue.push_back((right, depth + 1));
        }
    }

    0
}

impl Solution {
    #[must_use]
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        min_depth(root)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0111::min_depth, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            min_depth(to_bt(&[
                3.into(),
                9.into(),
                20.into(),
                None,
                None,
                15.into(),
                7.into()
            ])),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            min_depth(to_bt(&[
                2.into(),
                None,
                3.into(),
                None,
                4.into(),
                None,
                5.into(),
                None,
                6.into(),
            ])),
            5
        );
    }
}
