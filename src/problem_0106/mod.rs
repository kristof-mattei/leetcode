use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::shared::TreeNode;

fn calc(
    inorder_val_to_index: &BTreeMap<i32, usize>,
    postorder: &mut Vec<i32>,
    left: usize,
    right: usize,
) -> Rc<RefCell<TreeNode>> {
    let mut node = TreeNode::new(postorder.pop().unwrap());

    let inorder_index = inorder_val_to_index[&node.val];

    if inorder_index < right {
        node.right = Some(calc(
            inorder_val_to_index,
            postorder,
            inorder_index + 1,
            right,
        ));
    }

    if left < inorder_index {
        node.left = Some(calc(
            inorder_val_to_index,
            postorder,
            left,
            inorder_index - 1,
        ));
    };

    Rc::new(RefCell::new(node))
}

#[allow(clippy::unnecessary_wraps)]
fn build_tree(inorder: &[i32], mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let inorder_val_to_index = inorder
        .iter()
        .enumerate()
        .map(|(p, &v)| (v, p))
        .collect::<BTreeMap<i32, usize>>();

    Some(calc(
        &inorder_val_to_index,
        &mut postorder,
        0,
        inorder.len() - 1,
    ))
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build_tree(&inorder, postorder)
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use crate::{problem_0106::build_tree, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            build_tree(&[9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            to_bt(&[
                3.into(),
                9.into(),
                20.into(),
                None,
                None,
                15.into(),
                7.into()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            build_tree(&[3, 2, 1], vec![3, 2, 1]),
            to_bt(&[1.into(), 2.into(), None, 3.into()])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            build_tree(&[2, 3, 1], vec![3, 2, 1]),
            to_bt(&[1.into(), 2.into(), None, None, 3.into()])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            build_tree(&[1, 2, 3, 4], vec![1, 4, 3, 2]),
            to_bt(&[2.into(), 1.into(), 3.into(), None, None, None, 4.into()])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            build_tree(&[1, 2, 3, 4], vec![2, 3, 1, 4]),
            to_bt(&[4.into(), 1.into(), None, None, 3.into(), 2.into()])
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            build_tree(&[1, 2, 3, 4, 5], vec![1, 4, 5, 3, 2]),
            to_bt(&[
                2.into(),
                1.into(),
                3.into(),
                None,
                None,
                None,
                5.into(),
                4.into()
            ])
        );
    }
}
