use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        binary_tree_paths(root)
    }
}

fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut done_paths = vec![];
    let start = format!("{}", root.as_ref().unwrap().borrow().val);
    let mut todo = vec![(root, start)];

    while let Some((node, history)) = todo.pop() {
        let borrowed = node.as_ref().unwrap().borrow();

        if borrowed.left.is_none() && borrowed.right.is_none() {
            done_paths.push(history);
            continue;
        }

        if let Some(left) = borrowed.left.clone() {
            let mut history = history.clone();
            history.push_str(format!("->{}", left.borrow().val).as_str());

            todo.push((Some(left), history));
        }

        if let Some(right) = borrowed.right.clone() {
            let mut history = history.clone();
            history.push_str(format!("->{}", right.borrow().val).as_str());

            todo.push((Some(right), history));
        }
    }

    done_paths
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0257::binary_tree_paths;
    use crate::shared::to_bt;

    #[test]
    fn test_1() {
        let mut result = binary_tree_paths(to_bt(&[1.into(), 2.into(), 3.into(), None, 5.into()]));
        result.sort_unstable();

        let mut expected = ["1->2->5", "1->3"];
        expected.sort_unstable();

        assert_eq!(result, expected);
    }
}
