use std::{cell::RefCell, rc::Rc};

use crate::shared::TreeNode;

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p?;
        let q = q?;

        if let LeastCommonAncestor::Found(v) = lowest_common_ancestor(&root, p, q) {
            v
        } else {
            None
        }
    }
}

enum LeastCommonAncestor {
    None,
    One,
    Found(Option<Rc<RefCell<TreeNode>>>),
}

#[allow(clippy::needless_pass_by_value)]
fn lowest_common_ancestor(
    node: &Option<Rc<RefCell<TreeNode>>>,
    p: Rc<RefCell<TreeNode>>,
    q: Rc<RefCell<TreeNode>>,
) -> LeastCommonAncestor {
    if node.is_none() {
        return LeastCommonAncestor::None;
    }

    let node = node.as_ref().unwrap();

    let node_borrowed = node.borrow();

    let from_left = lowest_common_ancestor(&node_borrowed.left, p.clone(), q.clone());

    if matches!(from_left, LeastCommonAncestor::Found(_)) {
        return from_left;
    }

    let from_right = lowest_common_ancestor(&node_borrowed.right, p.clone(), q.clone());

    if matches!(from_right, LeastCommonAncestor::Found(_)) {
        return from_right;
    }

    let this_matches = node_borrowed.val == p.borrow().val || node_borrowed.val == q.borrow().val;

    match (from_left, from_right, this_matches) {
        (LeastCommonAncestor::None, LeastCommonAncestor::None, false) => LeastCommonAncestor::None,
        (LeastCommonAncestor::None, LeastCommonAncestor::None, true)
        | (LeastCommonAncestor::None, LeastCommonAncestor::One, false)
        | (LeastCommonAncestor::One, LeastCommonAncestor::None, false) => LeastCommonAncestor::One,
        (LeastCommonAncestor::None, LeastCommonAncestor::One, true)
        | (LeastCommonAncestor::One, LeastCommonAncestor::None, true)
        | (LeastCommonAncestor::One, LeastCommonAncestor::One, false) => {
            LeastCommonAncestor::Found(Some(Rc::clone(node)))
        },
        (LeastCommonAncestor::One, LeastCommonAncestor::One, true)
        | (_, LeastCommonAncestor::Found(_), _)
        | (LeastCommonAncestor::Found(_), _, _) => {
            panic!("Should have been checked before")
        },
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        problem_0236::{lowest_common_ancestor, LeastCommonAncestor},
        shared::{to_bt, TreeNode},
    };

    #[test]
    fn test_1() {
        let result = lowest_common_ancestor(
            &to_bt(&[
                3.into(),
                5.into(),
                1.into(),
                6.into(),
                2.into(),
                0.into(),
                8.into(),
                None,
                None,
                7.into(),
                4.into(),
            ]),
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

        assert!(
            matches!(result, LeastCommonAncestor::Found(ref v) if v.as_ref().unwrap().borrow().val == 3)
        );
    }
}
