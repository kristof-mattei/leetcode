use std::cell::RefCell;
use std::rc::Rc;

use shared::TreeNode;

impl Solution {
    #[must_use]
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        let p = p?;
        let q = q?;

        lowest_common_ancestor(root, p, q)
    }
}

fn lowest_common_ancestor(
    root: Rc<RefCell<TreeNode>>,
    p: Rc<RefCell<TreeNode>>,
    q: Rc<RefCell<TreeNode>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root_borrowed = root.borrow();

    let root_value = root_borrowed.val;
    let p_value = p.borrow().val;
    let q_value = q.borrow().val;

    if root_value < p_value && root_value < q_value {
        lowest_common_ancestor(Rc::clone(root_borrowed.right.as_ref().unwrap()), p, q)
    } else if root_value > p_value && root_value > q_value {
        lowest_common_ancestor(Rc::clone(root_borrowed.left.as_ref().unwrap()), p, q)
    } else {
        drop(root_borrowed);
        Some(root)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use shared::{TreeNode, to_bt};

    use crate::problem_0235::lowest_common_ancestor;

    #[test]
    fn test_1() {
        let result = lowest_common_ancestor(
            to_bt(&[
                6.into(),
                2.into(),
                8.into(),
                0.into(),
                4.into(),
                7.into(),
                9.into(),
                None,
                None,
                3.into(),
                5.into(),
            ])
            .unwrap(),
            Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: None,
            })),
        );

        let result = result.unwrap();
        let result = result.borrow();

        assert_eq!(result.val, 6);
    }
}
