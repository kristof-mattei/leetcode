use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::{Solution, TreeNode};

fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    generate_trees_r(1, n as usize)
}

fn generate_trees_r(start: usize, end: usize) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if start > end {
        return vec![None];
    }

    let mut trees = vec![];

    for i in start..=end {
        let left = generate_trees_r(start, i - 1);
        let right = generate_trees_r(i + 1, end);

        for l in &left {
            for r in &right {
                trees.push(Some(Rc::new(RefCell::new(TreeNode {
                    val: (i as i32),
                    left: l.clone(),
                    right: r.clone(),
                }))));
            }
        }
    }

    trees
}

impl Solution {
    #[must_use]
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        generate_trees(n)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{problem_0095::generate_trees, shared::from_bt};

    #[test]
    fn test_1() {
        let expected = vec![
            vec![1.into(), None, 2.into(), None, 3.into()],
            vec![1.into(), None, 3.into(), 2.into()],
            vec![2.into(), 1.into(), 3.into()],
            vec![3.into(), 1.into(), None, None, 2.into()],
            vec![3.into(), 2.into(), None, 1.into()],
        ];

        let result = generate_trees(3);

        let easy_list = result
            .into_iter()
            .map(from_bt)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        assert_eq!(easy_list, expected);
    }
}
