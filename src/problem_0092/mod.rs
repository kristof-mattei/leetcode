use crate::shared::ListNode;

fn reverse_between(
    mut current: Option<Box<ListNode>>,
    left: usize,
    right: usize,
) -> Option<Box<ListNode>> {
    if left == right || current.as_ref().unwrap().next.is_none() {
        return current;
    }

    let mut previous = None;

    let mut index = 1;

    let mut section1 = None;
    let mut section3 = None;

    while let Some(mut node) = current {
        current = node.next.take();

        if index == left {
            node.next = None;
            section1 = previous;
        } else if index == right {
            section3 = current.take();
            node.next = previous;
        } else {
            node.next = previous;
        }

        previous = Some(node);
        index += 1;
    }

    if section3.is_none() && section1.is_none() {
        previous
    } else if section1.is_none() {
        let mut connector = &mut previous;

        while let Some(c) = connector {
            connector = &mut c.next;
        }

        *connector = section3;

        previous
    } else {
        let mut orig_section1 = None;

        while let Some(mut node) = section1 {
            section1 = node.next.take();
            node.next = orig_section1;
            orig_section1 = Some(node);
        }

        let mut connector = &mut orig_section1;
        while let Some(c) = connector {
            connector = &mut c.next;
        }

        *connector = previous;

        while let Some(c) = connector {
            connector = &mut c.next;
        }
        *connector = section3;

        orig_section1
    }
}

impl Solution {
    #[must_use]
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        reverse_between(head, left as usize, right as usize)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0092::reverse_between, shared::to_ll};

    #[test]
    fn test_1() {
        let input = to_ll(&[1, 2, 3, 4, 5]);

        let expected = to_ll(&[1, 4, 3, 2, 5]);

        assert_eq!(reverse_between(input, 2, 4), expected);
    }

    #[test]
    fn test_2() {
        let input = to_ll(&[5]);

        let expected = to_ll(&[5]);

        assert_eq!(reverse_between(input, 1, 1), expected);
    }
}
