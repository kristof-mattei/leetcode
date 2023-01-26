use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::shared::TreeNode;

fn level_order_bottom(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if let Some(root_r) = root {
        let b = root_r.borrow();

        let mut left = level_order_bottom(b.left.as_ref());
        let mut right = level_order_bottom(b.right.as_ref());

        let left_len = left.len();
        let right_len = right.len();

        if left_len > right_len {
            let offset = left_len - right_len;

            for i in 0..right_len {
                left[offset + i].append(&mut right[i]);
            }

            left.push(vec![b.val]);
            left
        } else {
            let offset = right_len - left_len;

            for i in 0..left_len {
                mem::swap(&mut right[offset + i], &mut left[i]);
                right[offset + i].append(&mut left[i]);
            }

            right.push(vec![b.val]);
            right
        }
    } else {
        vec![]
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        level_order_bottom(root.as_ref())
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0107::level_order_bottom;
    use crate::shared::to_bt;

    #[test]
    fn test_1() {
        assert_eq!(
            level_order_bottom(
                to_bt(&[
                    3.into(),
                    9.into(),
                    20.into(),
                    None,
                    None,
                    15.into(),
                    7.into()
                ])
                .as_ref()
            ),
            [vec![15, 7], vec![9, 20], vec![3]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            level_order_bottom(
                to_bt(&[
                    3.into(),
                    9.into(),
                    20.into(),
                    12.into(),
                    31.into(),
                    15.into(),
                    7.into(),
                    None,
                    1.into(),
                    6.into()
                ])
                .as_ref()
            ),
            [vec![1, 6], vec![12, 31, 15, 7], vec![9, 20], vec![3]]
        );
    }
}
