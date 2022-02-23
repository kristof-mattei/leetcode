pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

#[must_use]
pub fn to_ll(input: &[i32]) -> Option<Box<ListNode>> {
    if input.is_empty() {
        return None;
    }

    Some(Box::new(ListNode {
        val: input[0],
        next: to_ll(&input[1..]),
    }))
}
