use std::collections::HashMap;

use crate::shared::Solution;

impl Solution {
    #[must_use]
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        majority_element(nums)
    }
}

fn majority_element(nums: Vec<i32>) -> i32 {
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let len = (nums.len() / 2) + 1;

    for n in nums {
        let val = hm.entry(n).and_modify(|x| *x += 1).or_insert(1);

        if *val > len {
            return n;
        }
    }

    *hm.iter().max_by_key(|(_, c)| *c).unwrap().0
}

#[cfg(test)]
mod test {
    use crate::problem_169::majority_element;



    #[test]
    fn test() {
        let vec = vec![1, 2, 2, 3];
        assert_eq!(majority_element(vec), 2);
    }

    #[test]
    fn test_2() {
        let vec = vec![1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(majority_element(vec), 1);
    }
}
