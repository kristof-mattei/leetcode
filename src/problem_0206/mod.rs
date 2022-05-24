use crate::shared::ListNode;

fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_some() {
        let mut rest = head.as_mut().unwrap().next.take();
        while rest.is_some() {
            let new_rest = rest.as_mut().unwrap().next.take();

            let mut new_head = rest;

            new_head.as_mut().unwrap().next = head;

            head = new_head;
            rest = new_rest;
        }
    }

    head
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        reverse_list(head)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::shared::to_ll;

    use super::reverse_list;

    #[test]
    fn test_1() {
        assert_eq!(reverse_list(to_ll(&[1, 2, 3])), to_ll(&[3, 2, 1]));
    }
}
