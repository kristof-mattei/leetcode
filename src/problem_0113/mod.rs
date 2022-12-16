use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

fn path_sum_h(
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
    mut history: Vec<i32>,
) -> Vec<Vec<i32>> {
    if let Some(r) = root {
        let b = r.borrow();

        history.push(b.val);

        if b.left.is_none() && b.right.is_none() {
            if target_sum == b.val {
                vec![history]
            } else {
                vec![]
            }
        } else {
            let mut left_result = path_sum_h(b.left.clone(), target_sum - b.val, history.clone());
            let mut right_result = path_sum_h(b.right.clone(), target_sum - b.val, history);
            left_result.append(&mut right_result);
            left_result
        }
    } else {
        vec![]
    }
}

fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    path_sum_h(root, target_sum, vec![])
}

impl Solution {
    #[must_use]
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        path_sum(root, target_sum)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0113::path_sum;
    use crate::shared::to_bt;

    #[test]
    fn test_1() {
        assert_eq!(
            path_sum(
                to_bt(&[
                    5.into(),
                    4.into(),
                    8.into(),
                    11.into(),
                    None,
                    13.into(),
                    4.into(),
                    7.into(),
                    2.into(),
                    None,
                    None,
                    5.into(),
                    1.into()
                ]),
                22
            ),
            [vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }

    #[test]
    fn test_2() {
        let expected: [Vec<i32>; 0] = [];
        assert_eq!(
            path_sum(to_bt(&[1.into(), 2.into(), 3.into(),]), 5),
            expected
        );
    }

    #[test]
    fn test_3() {
        let expected: [Vec<i32>; 0] = [];
        assert_eq!(path_sum(to_bt(&[1.into(), 2.into()]), 0), expected);
    }
}
