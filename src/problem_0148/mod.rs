use std::ops::Not;

use crate::shared::{ListNode, Solution};

enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut left_dummy = Some(Box::new(ListNode::new(0)));
    let mut right_dummy = Some(Box::new(ListNode::new(0)));

    let mut side = Side::Left;

    let mut left_ptr = &mut left_dummy;
    let mut right_ptr = &mut right_dummy;

    while let Some(mut next_node) = head {
        head = next_node.next.take();

        match side {
            Side::Left => {
                left_ptr.as_mut().unwrap().next = Some(next_node);
                left_ptr = &mut left_ptr.as_mut().unwrap().next;
            },
            Side::Right => {
                right_ptr.as_mut().unwrap().next = Some(next_node);
                right_ptr = &mut right_ptr.as_mut().unwrap().next;
            },
        };

        side = !side;
    }

    (left_dummy.unwrap().next, right_dummy.unwrap().next)
}

fn merge_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(mut l), Some(mut r)) => {
            if l.val < r.val {
                l.next = merge_lists(l.next, Some(r));
                Some(l)
            } else {
                r.next = merge_lists(Some(l), r.next);
                Some(r)
            }
        },
        (l @ Some(_), None) => l,
        (None, r @ Some(_)) => r,
        (None, None) => None,
    }
}

fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match split(head) {
        (l @ Some(_), r @ Some(_)) => merge_lists(sort_list(l), sort_list(r)),
        (l @ Some(_), None) => l,
        (None, r @ Some(_)) => r,
        (None, None) => None,
    }
}

impl Solution {
    #[must_use]
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        sort_list(head)
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_0148::sort_list, shared::to_ll};

    #[test]
    fn test_1() {
        assert_eq!(
            sort_list(to_ll(&[-1, 5, 3, 4, 0])),
            to_ll(&[-1, 0, 3, 4, 5])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(sort_list(to_ll(&[])), to_ll(&[]));
    }
}
