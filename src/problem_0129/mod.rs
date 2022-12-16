use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

fn sum_numbers_h(root: &Option<Rc<RefCell<TreeNode>>>, current_sum: i32) -> i32 {
    if let Some(r) = root {
        let b = r.borrow();

        let current_sum = current_sum * 10 + b.val;

        if b.left.is_none() && b.right.is_none() {
            return current_sum;
        }
        sum_numbers_h(&b.left, current_sum) + sum_numbers_h(&b.right, current_sum)
    } else {
        0
    }
}

fn sum_numbers(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    sum_numbers_h(root, 0)
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        sum_numbers(&root)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0129::sum_numbers;
    use crate::shared::to_bt;

    #[test]
    fn test_1() {
        assert_eq!(sum_numbers(&to_bt(&[1.into(), 2.into(), 3.into()])), 25);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            sum_numbers(&to_bt(&[4.into(), 9.into(), 0.into(), 5.into(), 1.into()])),
            1026
        );
    }
}
