use crate::shared::ListNode;

fn merge_two_lists(
    head1: Option<Box<ListNode>>,
    head2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (head1, head2) {
        (None, None) => None,
        (None, Some(n)) | (Some(n), None) => Some(n),
        (Some(mut l), Some(mut r)) => {
            let n = if l.val <= r.val {
                l.next = merge_two_lists(l.next, Some(r));
                l
            } else {
                r.next = merge_two_lists(Some(l), r.next);
                r
            };

            Some(n)
        },
    }
}

impl Solution {
    #[must_use]
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        merge_two_lists(list1, list2)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0021::merge_two_lists, shared::to_ll};

    #[test]
    fn test() {
        assert_eq!(
            merge_two_lists(to_ll(&[1, 2, 4]), to_ll(&[1, 3, 4])),
            to_ll(&[1, 1, 2, 3, 4, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(merge_two_lists(to_ll(&[]), to_ll(&[])), to_ll(&[]));
    }

    #[test]
    fn test_6() {
        assert_eq!(merge_two_lists(to_ll(&[]), to_ll(&[6])), to_ll(&[6]));
    }
}
