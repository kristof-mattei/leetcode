use crate::shared::ListNode;

impl Solution {
    #[must_use]
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        odd_even_list(head)
    }
}

enum EvenOdd {
    Even,
    Odd,
}

impl EvenOdd {
    fn flip(&mut self) {
        *self = match *self {
            EvenOdd::Even => EvenOdd::Odd,
            EvenOdd::Odd => EvenOdd::Even,
        };
    }
}

fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list1 = head;
    let mut list2 = None;

    let mut list1_tail = &mut list1;
    let mut list2_tail = &mut list2;

    let mut even_odd = EvenOdd::Odd;

    loop {
        match even_odd {
            EvenOdd::Odd => {
                let Some(n) = list1_tail else {
                    break;
                };

                list1_tail = &mut n.next;

                std::mem::swap(list1_tail, list2_tail);
            },
            EvenOdd::Even => {
                let Some(n) = list2_tail else {
                    break;
                };

                list2_tail = &mut n.next;

                std::mem::swap(list1_tail, list2_tail);
            },
        }

        even_odd.flip();
    }

    // Point back to the beginning
    list2_tail = &mut list2;

    // list1_tail contains None, and we put the list2_tail, which is the start of list2
    std::mem::swap(list1_tail, list2_tail);

    list1
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0328::odd_even_list;
    use crate::shared::to_ll;

    #[test]
    fn test_1() {
        assert_eq!(
            odd_even_list(to_ll(&[1, 2, 3, 4, 5])),
            to_ll(&[1, 3, 5, 2, 4])
        );
    }
    #[test]

    fn test_2() {
        assert_eq!(
            odd_even_list(to_ll(&[2, 1, 3, 5, 6, 4, 7])),
            to_ll(&[2, 3, 6, 7, 1, 5, 4])
        );
    }
}
