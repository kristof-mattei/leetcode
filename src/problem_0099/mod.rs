use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::{Solution, TreeNode};
fn recover_tree(_root: &mut Option<Rc<RefCell<TreeNode>>>) {}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        recover_tree(root);
    }
}

#[cfg(test)]
mod tests {

    use crate::{problem_0099::recover_tree, shared::to_bt};

    #[test]
    fn test_1() {
        let mut input = to_bt(&[1.into(), 3.into(), None, None, 2.into()]);
        let expected = to_bt(&[3.into(), 1.into(), None, None, 2.into()]);
        recover_tree(&mut input);
        assert_eq!(input, expected);
    }
}
