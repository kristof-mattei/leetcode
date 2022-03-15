use crate::shared::{ListNode, Solution};

fn reverse_between(
    head: Option<Box<ListNode>>,
    left: usize,
    right: usize,
) -> Option<Box<ListNode>> {
    if left == right || head.as_ref().unwrap().next.is_none() {
        return head;
    }

    let mut previous = None;

    let mut processor = head;

    let mut index = 1;

    let mut section1 = None;
    let mut section3 = None;

    while processor.as_ref().is_some() {
        let mut node = processor.unwrap();

        processor = node.next.take();

        if index == left {
            node.next = None;
            section1 = previous;
        } else if index == right {
            section3 = processor.take();
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
        let mut reversed_section2 = previous;

        let mut connector = reversed_section2.as_mut().unwrap();

        while connector.next.is_some() {
            connector = connector.next.as_mut().unwrap();
        }
        connector.next = section3;

        reversed_section2
    } else {
        let mut orig_section1 = None;

        while section1.is_some() {
            let mut node = section1.unwrap();
            section1 = node.next.take();
            node.next = orig_section1;
            orig_section1 = Some(node);
        }

        let mut connector = orig_section1.as_mut().unwrap();

        while connector.next.is_some() {
            connector = connector.next.as_mut().unwrap();
        }

        connector.next = previous;

        while connector.next.is_some() {
            connector = connector.next.as_mut().unwrap();
        }

        connector.next = section3;

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
