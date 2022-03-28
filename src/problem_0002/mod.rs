use crate::shared::{ListNode, Solution};

fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry = 0;

    let mut l1_ref = &mut l1;

    while !(l1_ref.is_none() && l2.is_none() && carry == 0) {
        let val1 = l1_ref.as_ref().map_or(0, |l| l.val);
        let val2 = l2.as_ref().map_or(0, |l| l.val);

        let val = carry + val1 + val2;

        let r = val % 10;

        carry = val / 10;

        let n = l1_ref.get_or_insert(Box::new(ListNode::new(r)));
        n.val = r;

        l1_ref = &mut n.next;
        l2 = l2.and_then(|l| l.next);
    }

    l1
}

impl Solution {
    #[must_use]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        add_two_numbers(l1, l2)
    }
}

#[cfg(test)]
mod tests {
    use crate::{problem_0002::add_two_numbers, shared::to_ll};

    #[test]
    fn test_1() {
        assert_eq!(
            add_two_numbers(to_ll(&[2, 4, 3]), to_ll(&[5, 6, 4])),
            to_ll(&[7, 0, 8])
        );
    }
    #[test]

    fn test_2() {
        assert_eq!(add_two_numbers(to_ll(&[0]), to_ll(&[0])), to_ll(&[0]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            add_two_numbers(to_ll(&[9, 9, 9, 9, 9, 9, 9]), to_ll(&[9, 9, 9, 9])),
            to_ll(&[8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
