use crate::shared::{Solution, TreeNode};
use std::{cell::RefCell, rc::Rc};

fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {}

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        flatten(root);
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_0114::flatten, shared::to_bt};

    #[test]
    fn test_1() {
        let mut tree = to_bt(&[
            1.into(),
            2.into(),
            5.into(),
            3.into(),
            4.into(),
            None,
            6.into(),
        ]);

        flatten(&mut tree);

        assert_eq!(
            tree,
            to_bt(&[1.into(), 2.into(), 3.into(), 4.into(), 5.into(), 6.into()])
        );
    }

    #[test]
    fn test_2() {
        let mut tree = to_bt(&[]);

        flatten(&mut tree);

        assert_eq!(tree, to_bt(&[]));
    }

    #[test]
    fn test_3() {
        let mut tree = to_bt(&[0.into()]);

        flatten(&mut tree);

        assert_eq!(tree, to_bt(&[0.into()]));
    }
}
