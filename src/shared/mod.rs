pub struct Solution {}

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

// impl ListNode {
//     #[inline]
//     pub(crate) fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::{cell::RefCell, collections::VecDeque, option::Option, rc::Rc};

#[must_use]
pub fn to_ll(input: &[i32]) -> Option<Box<ListNode>> {
    if input.is_empty() {
        return None;
    }

    Some(Box::new(ListNode {
        val: input[0],
        next: to_ll(&input[1..]),
    }))
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

/// Converts a slice of &[Option<T>] to a Binary tree
///
/// # Examples
///
/// ```
/// use leet_code::shared::to_bt;
/// use leet_code::shared::tn;
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
    let mut index = 1;
    let mut queue = VecDeque::from_iter([root.clone()]);

    while !queue.is_empty() && index < input.len() {
        let current = queue.pop_front().unwrap().unwrap();

        if index < input.len() {
            let item = input.get(index).and_then(Option::as_ref);
            index += 1;

            if let Some(&v) = item {
                let node = tn(v, None, None);
                (*current.borrow_mut()).left = node.clone();
                queue.push_back(node);
            }
        }

        if index < input.len() {
            let item = input.get(index).and_then(Option::as_ref);
            index += 1;

            if let Some(&v) = item {
                let node = tn(v, None, None);
                (*current.borrow_mut()).right = node.clone();
                queue.push_back(node);
            }
        }
    }

    root
}

/// Converts a Binary tree to a flat representation
///
/// # Examples
///
/// ```
/// use leet_code::shared::from_bt;
/// use leet_code::shared::tn;
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
                results.push(Some(current.borrow().val));

                let left = current.borrow_mut().left.take();
                queue.push_back(left);

                let right = current.borrow_mut().right.take();
                queue.push_back(right);
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
    use crate::shared::{from_bt, tn, to_bt};

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
    #[test]

    fn test_from_bt_1() {
        let expected = [
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

        let input = tn(
            5,
            tn(4, tn(3, tn(-1, None, None), None), None),
            tn(7, tn(2, tn(9, None, None), None), None),
        );

        assert_eq!(from_bt(input), expected);
    }
}
