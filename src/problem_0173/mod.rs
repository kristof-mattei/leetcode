use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

struct BSTIterator(Vec<Rc<RefCell<TreeNode>>>);

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self(vec![root.unwrap()])
    }

    fn next(&mut self) -> i32 {
        let current = self.0.pop().unwrap();

        let left = current.borrow_mut().left.take();

        if let Some(l) = left {
            self.0.push(current);
            self.0.push(l);
            return self.next();
        }

        let val = current.borrow().val;

        let right = current.borrow_mut().right.take();

        if let Some(r) = right {
            self.0.push(r);
        }

        val
    }

    fn has_next(&self) -> bool {
        !self.0.is_empty()
    }
}

impl Solution {
    pub fn problem() {
        let mut iterator = BSTIterator::new(None);
        let _ = iterator.next();
        let _ = iterator.has_next();
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::BSTIterator;
    use crate::shared::to_bt;

    #[test]
    fn test_1() {
        let mut iterator = BSTIterator::new(to_bt(&[
            Some(7),
            Some(3),
            Some(15),
            None,
            None,
            Some(9),
            Some(20),
        ]));

        assert_eq!(iterator.next(), 3);
        assert_eq!(iterator.next(), 7);
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 9);
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 15);
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), 20);
        assert!(!iterator.has_next());
    }
}
