pub struct Solution {}

use std::{cell::RefCell, collections::VecDeque, option::Option, rc::Rc};

#[derive(PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)?;

        if let Some(n) = &self.next {
            write!(f, ",{:?}", n)?;
        }

        Ok(())
    }
}

impl ListNode {
    #[inline]
    #[must_use]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[must_use]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[must_use]
pub fn to_ll(input: &[i32]) -> Option<Box<ListNode>> {
    if let [head, rest @ ..] = input {
        Some(Box::new(ListNode {
            val: *head,
            next: to_ll(rest),
        }))
    } else {
        None
    }
}

#[must_use]
pub fn vec_eq<T>(left: Vec<T>, mut right: Vec<T>) -> bool
where
    T: std::cmp::Eq,
{
    if left.len() != right.len() {
        return false;
    }

    for l in left {
        if let Some(p) = right.iter().position(|x| x == &l) {
            right.remove(p);
        } else {
            return false;
        }
    }

    true
}

pub fn sort_vec_of_vec<T>(vec: &mut Vec<Vec<T>>)
where
    T: std::cmp::Ord,
{
    for inner_v in vec.iter_mut() {
        inner_v.sort_unstable();
    }

    vec.sort_unstable();
}

#[derive(PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

/// Converts a slice of &[Option<T>] to a Binary tree
///
/// # Examples
///
/// ```
/// use leetcode::shared::to_bt;
/// use leetcode::shared::tn;
///
/// let input = [1.into(), None, 3.into()];
/// assert_eq!(to_bt(&input), tn(1, None, tn(3, None, None)));
/// ```
///
/// # Panics
///
/// Panics if cannot borrow Rc
#[must_use]
pub fn to_bt(input: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if input.is_empty() {
        return None;
    }

    let root = tn(input[0].unwrap(), None, None);
    let mut queue = VecDeque::from_iter([root.clone()]);

    let mut side = Side::Left;

    for o in input.iter().skip(1) {
        let node = queue.front().unwrap().as_ref().unwrap();

        if let Some(&v) = o.as_ref() {
            let new_node = Some(Rc::new(RefCell::new(TreeNode::new(v))));

            match side {
                Side::Left => {
                    (*node.borrow_mut()).left = new_node.clone();
                },
                Side::Right => {
                    (*node.borrow_mut()).right = new_node.clone();
                },
            }

            queue.push_back(new_node);
        }

        if side == Side::Left {
            side = Side::Right;
        } else {
            queue.pop_front();

            side = Side::Left;
        }
    }

    root
}

/// Converts a Binary tree to a flat representation
///
/// # Examples
///
/// ```
/// use leetcode::shared::from_bt;
/// use leetcode::shared::tn;
///
/// let input = tn(1, None, tn(3, None, None));
/// assert_eq!(from_bt(input), [1.into(), None, 3.into()]);
/// ```
///
/// # Panics
///
/// Panics if cannot borrow Rc
#[must_use]
pub fn from_bt(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut results = vec![];
    let mut queue = VecDeque::from_iter([root]);

    while !queue.is_empty() && queue.iter().any(Option::is_some) {
        match queue.pop_front().unwrap() {
            None => {
                results.push(None);
            },
            Some(current) => {
                let borrow = current.borrow();
                results.push(Some(borrow.val));

                queue.push_back(borrow.left.clone());
                queue.push_back(borrow.right.clone());
            },
        }
    }

    results
}

#[must_use]
pub fn tn(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

#[cfg(test)]
mod tests {
    use crate::shared::{tn, to_bt};

    #[test]
    fn test_bt() {
        let input = [
            5.into(),
            4.into(),
            7.into(),
            3.into(),
            None,
            2.into(),
            None,
            (-1).into(),
            None,
            9.into(),
        ];

        let expected = tn(
            5,
            tn(4, tn(3, tn(-1, None, None), None), None),
            tn(7, tn(2, tn(9, None, None), None), None),
        );

        assert_eq!(to_bt(&input), expected);
    }

    #[test]
    fn test_bt_2() {
        let input = (1..=15).into_iter().map(Some).collect::<Vec<_>>();

        let expected = tn(
            1,
            tn(
                2,
                tn(4, tn(8, None, None), tn(9, None, None)),
                tn(5, tn(10, None, None), tn(11, None, None)),
            ),
            tn(
                3,
                tn(6, tn(12, None, None), tn(13, None, None)),
                tn(7, tn(14, None, None), tn(15, None, None)),
            ),
        );

        assert_eq!(to_bt(&input), expected);
    }
}
