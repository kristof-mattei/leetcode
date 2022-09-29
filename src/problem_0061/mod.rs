use crate::shared::ListNode;

fn get_last_n(
    head: &mut Option<Box<ListNode>>,
    rotate_places: usize,
    depth_counter: usize,
) -> (Option<Box<ListNode>>, usize, usize) {
    match head {
        | Some(n) => {
            let (r, depth, max_depth) = get_last_n(&mut n.next, rotate_places, depth_counter + 1);

            if (rotate_places % max_depth) == depth {
                let to_return = n.next.take();
                (to_return, depth + 1, max_depth)
            } else {
                (r, depth + 1, max_depth)
            }
        },
        | None => (None, 0, depth_counter),
    }
}

fn re_attach(
    new_head: Option<Box<ListNode>>,
    head: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match new_head {
        | Some(mut n) => {
            n.next = re_attach(n.next, head);
            Some(n)
        },
        | None => head,
    }
}

fn rotate_right(mut head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    let (new_head, ..) = get_last_n(&mut head, k, 0);

    re_attach(new_head, head)
}

impl Solution {
    #[must_use]
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        rotate_right(head, k as usize)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0061::rotate_right, shared::to_ll};

    #[test]
    fn test_1() {
        assert_eq!(
            rotate_right(to_ll(&[1, 2, 3, 4, 5]), 2),
            to_ll(&[4, 5, 1, 2, 3])
        );
    }
}
