use crate::shared::{ListNode, Solution};

fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_head = Some(Box::new(ListNode { val: 0, next: None }));

    let mut previous = &mut new_head;

    while let Some(mut c) = head.take() {
        head = c.next.take();

        while previous
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .map_or(false, |n| n.val < c.val)
        {
            previous = &mut previous.as_mut().unwrap().next;
        }

        c.next = previous.as_mut().unwrap().next.take();
        previous.as_mut().unwrap().next = Some(c);

        // reset
        previous = &mut new_head;
    }

    new_head.unwrap().next
}

impl Solution {
    #[must_use]
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        insertion_sort_list(head)
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_0147::insertion_sort_list, shared::to_ll};

    #[test]
    fn test_1() {
        assert_eq!(
            insertion_sort_list(to_ll(&[4, 2, 1, 3])),
            to_ll(&[1, 2, 3, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            insertion_sort_list(to_ll(&[-1, 5, 3, 4, 0])),
            to_ll(&[-1, 0, 3, 4, 5])
        );
    }
}
