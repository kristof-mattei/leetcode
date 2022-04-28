use crate::shared::ListNode;

fn delete_duplicates_r(head: &mut Option<Box<ListNode>>) {
    let h = match head {
        Some(h) => h,
        None => {
            return;
        },
    };

    let current_val = h.val;

    let mut next = h.next.take();
    while let Some(c) = next {
        let next_val = c.val;

        if next_val == current_val {
            next = c.next;
        } else {
            next = Some(c);
            break;
        }
    }

    delete_duplicates_r(&mut next);
    h.next = next;
}

impl Solution {
    #[must_use]
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        delete_duplicates_r(&mut head);
        head
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::{problem_0083::delete_duplicates_r, shared::to_ll};

    #[test]
    fn test_1() {
        let mut input = to_ll(&[1, 1, 2]);
        delete_duplicates_r(&mut input);
        assert_eq!(input, to_ll(&[1, 2]));
    }

    #[test]
    fn test_2() {
        let mut input = to_ll(&[1, 1, 2, 3, 3]);
        delete_duplicates_r(&mut input);
        assert_eq!(input, to_ll(&[1, 2, 3]));
    }

    #[test]
    fn test_3() {
        let mut input = to_ll(&[]);
        delete_duplicates_r(&mut input);
        assert_eq!(input, to_ll(&[]));
    }
}
