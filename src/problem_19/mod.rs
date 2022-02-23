use crate::shared::Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, usize) {
    match head {
        None => (None, 1),
        Some(mut node) => {
            let (next_of_next, distance_from_end) = remove_nth_from_end(node.next, n);
            // if we are at distance from end, we
            // return the next_of_next instead
            // of ourselves, skipping ourselves
            if n == distance_from_end as i32 {
                (next_of_next, distance_from_end + 1)
            } else {
                // update the next, which usually is just
                // next_of_next, except in cases where
                // next_of_next would point to the one we have to drop
                // and because of the condition above, we skip that one
                node.next = next_of_next;
                (Some(node), distance_from_end + 1)
            }
        },
    }
}

impl Solution {
    #[must_use]
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        remove_nth_from_end(head, n).0
    }
}

#[cfg(test)]
mod test {
    use crate::problem_19::remove_nth_from_end;

    use super::ListNode;


    fn to_ll(input: &[i32]) -> Option<Box<ListNode>> {
        if input.is_empty() {
            return None;
        }

        Some(Box::new(ListNode {
            val: input[0],
            next: to_ll(&input[1..]),
        }))
    }
    #[test]

    fn test() {
        assert_eq!(
            remove_nth_from_end(to_ll(&[1, 2, 3, 4, 5]), 2).0,
            to_ll(&[1, 2, 3, 5])
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            remove_nth_from_end(to_ll(&[1, 2, 3, 4, 5]), 3).0,
            to_ll(&[1, 2, 4, 5])
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(remove_nth_from_end(to_ll(&[1, 2]), 1).0, to_ll(&[1]));
    }
}
