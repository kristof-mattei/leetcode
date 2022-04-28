use std::iter;

use crate::shared::ListNode;

fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
    let length = iter::successors(head.as_ref(), |node| node.next.as_ref()).count();

    let half = (length + 1) / 2;

    let mut node = &mut *head;

    // run to half
    for _ in 0..half {
        node = &mut node.as_deref_mut().unwrap().next;
    }

    let mut reverse = None;
    let mut next = node.take();

    // reverse the rest
    while let Some(mut node) = next {
        next = std::mem::replace(&mut node.next, reverse);
        reverse = Some(node);
    }

    let mut normal = head.take();

    while let Some(mut n) = normal {
        normal = n.next.take();
        head = &mut head.insert(n).next;

        if let Some(mut r) = reverse {
            reverse = r.next.take();
            head = &mut head.insert(r).next;
        }
    }
}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        reorder_list(head);
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0143::reorder_list, shared::to_ll};

    #[test]
    fn test_1() {
        let mut list = to_ll(&[1, 2, 3, 4]);

        reorder_list(&mut list);

        assert_eq!(list, to_ll(&[1, 4, 2, 3]));
    }

    #[test]
    fn test_2() {
        let mut list = to_ll(&[1, 2, 3, 4, 5]);

        reorder_list(&mut list);

        assert_eq!(list, to_ll(&[1, 5, 2, 4, 3]));
    }

    #[test]
    fn test_3() {
        let mut list = to_ll(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        reorder_list(&mut list);

        assert_eq!(list, to_ll(&[1, 10, 2, 9, 3, 8, 4, 7, 5, 6]));
    }
}
