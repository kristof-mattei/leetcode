use std::cell::RefCell;
use std::rc::Rc;

use crate::shared::TreeNode;

fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let &mut Some(ref r) = root {
        let mut b = r.borrow_mut();

        match (b.left.take(), b.right.take()) {
            (None, None) => {},
            (l @ Some(_), None) => {
                b.right = l;
            },
            (None, r @ Some(_)) => {
                b.right = r;
            },
            (l @ Some(_), r @ Some(_)) => {
                b.right = l;

                let mut current = b.right.clone().unwrap();

                loop {
                    current = {
                        let borrow = current.borrow();
                        let right_ref = borrow.right.as_ref();

                        if let Some(r) = right_ref {
                            Rc::clone(r)
                        } else {
                            break;
                        }
                    };
                }
                current.borrow_mut().right = r;
            },
        }

        flatten(&mut b.right);
    }
}

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        flatten(root);
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0114::flatten;
    use crate::shared::to_bt;

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
            to_bt(&[
                1.into(),
                None,
                2.into(),
                None,
                3.into(),
                None,
                4.into(),
                None,
                5.into(),
                None,
                6.into()
            ])
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

    #[test]
    fn test_4() {
        let mut tree = to_bt(&[1.into(), 2.into(), None, 3.into()]);

        flatten(&mut tree);

        assert_eq!(tree, to_bt(&[1.into(), None, 2.into(), None, 3.into()]));
    }
}
