use crate::shared::ListNode;

fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }

    let mut first = head.unwrap();
    let mut second = first.next.take().unwrap();

    first.next = swap_pairs(second.next);
    second.next = Some(first);

    Some(second)
}

impl Solution {
    #[must_use]
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        swap_pairs(head)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0024::swap_pairs, shared::to_ll};

    #[test]
    fn test() {
        assert_eq!(swap_pairs(to_ll(&[1, 2, 3, 4])), to_ll(&[2, 1, 4, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(swap_pairs(to_ll(&[])), to_ll(&[]));
    }

    #[test]
    fn test_3() {
        assert_eq!(swap_pairs(to_ll(&[1])), to_ll(&[1]));
    }
}
