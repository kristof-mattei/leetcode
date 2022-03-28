use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    rc::Rc,
};

use crate::shared::{Solution, TreeNode};

fn calc(
    preorder: &mut VecDeque<i32>,
    inorder_val_to_index: &BTreeMap<i32, usize>,
    left: usize,
    right: usize,
) -> Rc<RefCell<TreeNode>> {
    let mut node = TreeNode::new(preorder.pop_front().unwrap());

    let inorder_index = inorder_val_to_index[&node.val];
    if left < inorder_index {
        node.left = Some(calc(
            preorder,
            inorder_val_to_index,
            left,
            inorder_index - 1,
        ));
    };

    if inorder_index < right {
        node.right = Some(calc(
            preorder,
            inorder_val_to_index,
            inorder_index + 1,
            right,
        ));
    }

    Rc::new(RefCell::new(node))
}

#[allow(clippy::unnecessary_wraps)]
fn build_tree(preorder: Vec<i32>, inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let inorder_val_to_index = inorder
        .iter()
        .enumerate()
        .map(|(p, &v)| (v, p))
        .collect::<BTreeMap<i32, usize>>();

    Some(calc(
        &mut VecDeque::from_iter(preorder),
        &inorder_val_to_index,
        0,
        inorder.len() - 1,
    ))
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build_tree(preorder, &inorder)
    }
}

#[cfg(test)]
mod test {
    use crate::{problem_0105::build_tree, shared::to_bt};

    #[test]
    fn test_1() {
        assert_eq!(
            build_tree(vec![3, 9, 20, 15, 7], &[9, 3, 15, 20, 7]),
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
            build_tree(vec![1, 2, 3], &[3, 2, 1]),
            to_bt(&[1.into(), 2.into(), None, 3.into()])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            build_tree(vec![1, 2, 3], &[2, 3, 1]),
            to_bt(&[1.into(), 2.into(), None, None, 3.into()])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            build_tree(vec![2, 1, 3, 4], &[1, 2, 3, 4]),
            to_bt(&[2.into(), 1.into(), 3.into(), None, None, None, 4.into()])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            build_tree(vec![4, 1, 3, 2], &[1, 2, 3, 4]),
            to_bt(&[4.into(), 1.into(), None, None, 3.into(), 2.into()])
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            build_tree(vec![2, 1, 3, 5, 4], &[1, 2, 3, 4, 5]),
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
