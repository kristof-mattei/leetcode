use crate::shared::ListNode;

fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if let Some(m) = lists.iter().flatten().min_by_key(|list| list.val) {
        let position = lists.iter().position(|v| v.as_ref() == Some(m)).unwrap();

        let mut head = lists.swap_remove(position).unwrap();

        if let Some(next) = head.next {
            lists.push(Some(next));
        }

        head.next = merge_k_lists(lists);

        Some(head)
    } else {
        None
    }
}

impl Solution {
    #[must_use]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        merge_k_lists(lists)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0023::merge_k_lists, shared::to_ll};

    #[test]
    fn test() {
        assert_eq!(merge_k_lists(vec![]), to_ll(&[]));
    }
    #[test]
    fn test_2() {
        assert_eq!(merge_k_lists(vec![to_ll(&[])]), to_ll(&[]));
    }
    #[test]
    fn test_3() {
        assert_eq!(
            merge_k_lists(vec![to_ll(&[1, 4, 5]), to_ll(&[1, 3, 4]), to_ll(&[2, 6])]),
            to_ll(&[1, 1, 2, 3, 4, 4, 5, 6])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            merge_k_lists(vec![to_ll(&[6, 8, 9]), to_ll(&[1, 8, 9])]),
            to_ll(&[1, 6, 8, 8, 9, 9])
        );
    }
}
