use crate::shared::ListNode;

fn delete_duplicates_r_mut(head: &mut Option<Box<ListNode>>) {
    let h = match head {
        | Some(h) => h,
        | None => {
            return;
        },
    };

    let current_val = h.val;

    let mut next = h.next.take();
    let mut delete_current = false;
    while let Some(c) = next {
        let next_val = c.val;

        if next_val == current_val {
            delete_current = true;
            next = c.next;
        } else {
            next = Some(c);
            break;
        }
    }

    delete_duplicates_r_mut(&mut next);
    if delete_current {
        *head = next;
    } else {
        h.next = next;
    }
}

// fn delete_duplicates_r(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     match head {
//         Some(mut h) => {
//             let current_val = h.val;

//             let mut next = h.next.take();
//             let mut delete_current = false;
//             while let Some(c) = next {
//                 let next_val = c.val;

//                 if next_val == current_val {
//                     delete_current = true;
//                     next = c.next;
//                 } else {
//                     next = Some(c);
//                     break;
//                 }
//             }

//             if delete_current {
//                 delete_duplicates_r(next)
//             } else {
//                 h.next = delete_duplicates_r(next);
//                 Some(h)
//             }
//         },
//         None => None,
//     }
// }
// fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     delete_duplicates_r(head)
// }

fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    delete_duplicates_r_mut(&mut head);
    head
}
impl Solution {
    #[must_use]
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        delete_duplicates(head)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::{problem_0082::delete_duplicates, shared::to_ll};

    #[test]
    fn test_1() {
        assert_eq!(
            delete_duplicates(to_ll(&[1, 2, 3, 3, 4, 4, 5])),
            to_ll(&[1, 2, 5])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(delete_duplicates(to_ll(&[1, 1, 1, 2, 3])), to_ll(&[2, 3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(delete_duplicates(to_ll(&[])), to_ll(&[]));
    }
}
