use crate::shared::ListNode;

fn add_to_end(head: &mut Option<Box<ListNode>>, to_add_to_end: Option<Box<ListNode>>) {
    let mut current = head;

    while let Some(mut c) = current.take() {
        if c.next.is_none() {
            c.next = to_add_to_end;
            let _ = current.insert(c); // put it back
            break;
        }

        current = &mut current.insert(c).next;
    }
}

fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut current = &mut head;

    while let Some(mut c) = current.take() {
        if c.val >= x {
            // nah, too big, next
            current = &mut current.insert(c).next;
        } else {
            // this one is less than x

            // return c (below x), head(above x) > n (don't care)

            if head.is_some() {
                add_to_end(&mut head, c.next.take());
                c.next = partition(head, x);
            } else {
                let n = c.next.take();
                c.next = partition(n, x);
            }

            head = Some(c);
            break;
        }
    }

    head
}

impl Solution {
    #[must_use]
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        partition(head, x)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0086::partition, shared::to_ll};

    #[test]
    fn test_1() {
        let input = to_ll(&[1, 4, 3, 2, 5, 2]);

        let result = partition(input, 3);

        assert_eq!(result, to_ll(&[1, 2, 2, 4, 3, 5]));
    }

    #[test]
    fn test_2() {
        let input = to_ll(&[2, 1]);

        let result = partition(input, 2);

        assert_eq!(result, to_ll(&[1, 2]));
    }

    #[test]
    fn test_3() {
        let input = to_ll(&[4, 1, 5, 2]);

        let result = partition(input, 3);

        assert_eq!(result, to_ll(&[1, 2, 4, 5]));
    }
}
