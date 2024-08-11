use crate::shared::ListNode;

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        is_palindrome(head)
    }
}

fn is_palindrome_r(
    length: usize,
    max_length: &mut usize,
    left: &mut &Option<Box<ListNode>>,
    node: Option<&ListNode>,
) -> bool {
    let node = node.unwrap();

    if let Some(next) = &node.next {
        if !is_palindrome_r(length + 1, max_length, left, Some(next)) {
            return false;
        }
    } else {
        *max_length = length;
    }
    if length <= (*max_length / 2) {
        return true;
    }

    let actual_left = left.as_ref().unwrap();

    if actual_left.val != node.val {
        return false;
    }

    *left = &actual_left.next;

    true
}

#[allow(clippy::needless_pass_by_value)]
fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    is_palindrome_r(0, &mut 0, &mut &head, head.as_deref())
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0234::is_palindrome, shared::to_ll};

    #[test]
    fn test_1() {
        assert!(is_palindrome(to_ll(&[1, 2, 2, 1])));
    }

    #[test]
    fn test_2() {
        assert!(is_palindrome(to_ll(&[1, 2, 3, 2, 1])));
    }

    #[test]
    fn test_3() {
        assert!(!is_palindrome(to_ll(&[1, 2])));
    }
}
