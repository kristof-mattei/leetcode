use crate::shared::{ListNode, Solution};

#[allow(clippy::vec_box)]
fn reverse_list(
    mut list: Vec<Box<ListNode>>,
    next: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match list.pop() {
        Some(mut t) => {
            t.next = reverse_list(list, next);
            Some(t)
        },
        None => next,
    }
}

fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // take k
    let mut list = Vec::new();
    for _ in 0..k {
        match head {
            Some(mut current) => {
                let next = current.next.take();
                list.push(current);

                head = next;
            },
            None => {
                return {
                    let mut previous = None;

                    while let Some(mut n) = list.pop() {
                        n.next = previous;
                        previous = Some(n);
                    }

                    previous
                }
            },
        }
    }

    let rest = reverse_k_group(head, k);

    reverse_list(list, rest)
}

impl Solution {
    #[must_use]
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        reverse_k_group(head, k)
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_25::reverse_k_group, shared::to_ll};

    #[test]
    fn test() {
        assert_eq!(
            reverse_k_group(to_ll(&[1, 2, 3, 4, 5]), 2),
            to_ll(&[2, 1, 4, 3, 5])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            reverse_k_group(to_ll(&[1, 2, 3, 4, 5]), 3),
            to_ll(&[3, 2, 1, 4, 5])
        );
    }
}
