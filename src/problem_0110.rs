use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

fn is_balanced(root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(root) = root {
        let mut r = root.borrow_mut();

        if is_balanced(r.left.as_ref()) && is_balanced(r.right.as_ref()) {
            let hl = r.left.as_ref().map_or(0, |x| x.borrow().val);
            let hr = r.right.as_ref().map_or(0, |x| x.borrow().val);

            r.val = 1 + std::cmp::max(hl, hr);

            (hl - hr).abs() <= 1
        } else {
            false
        }
    } else {
        true
    }
}

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_balanced(root.as_ref())
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0110::is_balanced;
    use crate::shared::to_bt;

    #[test]
    fn test_1() {
        assert!(is_balanced(
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
        ));
    }

    #[test]
    fn test_2() {
        assert!(!is_balanced(
            to_bt(&[
                1.into(),
                2.into(),
                2.into(),
                3.into(),
                3.into(),
                None,
                None,
                4.into(),
                4.into()
            ])
            .as_ref()
        ));
    }

    #[test]
    fn test_3() {
        assert!(is_balanced(
            to_bt(&[
                1.into(),
                2.into(),
                3.into(),
                4.into(),
                5.into(),
                6.into(),
                None,
                8.into(),
            ])
            .as_ref()
        ));
    }

    #[test]
    fn test_4() {
        assert!(!is_balanced(
            to_bt(&[1.into(), None, 2.into(), None, 3.into(),]).as_ref()
        ));
    }
}
