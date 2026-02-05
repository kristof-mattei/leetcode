use std::cmp::Reverse;

impl Solution {
    #[must_use]
    pub fn h_index(citations: Vec<i32>) -> i32 {
        h_index(citations)
    }
}

fn h_index(mut citations: Vec<i32>) -> i32 {
    citations.sort_unstable_by_key(|v| Reverse(*v));

    for i in 1..=citations.len() {
        if citations[i - 1] < i as i32 {
            return i as i32 - 1;
        }
    }

    citations.len() as i32
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0275::h_index;

    #[test]
    fn test_1() {
        assert_eq!(h_index(vec![3, 0, 6, 1, 5]), 3);
    }
}
