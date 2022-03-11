use crate::shared::Solution;

fn search_insert(nums: &[i32], target: i32) -> i32 {
    let mut offset = 0;
    let mut end = nums.len();

    let mut potential_insert = 0;

    while offset < end {
        let middle = (end - offset) / 2 + offset;
        match target.cmp(&nums[middle]) {
            std::cmp::Ordering::Equal => {
                return middle as i32;
            },
            std::cmp::Ordering::Greater => {
                // we narrow down the scope
                offset = middle + 1;
                potential_insert = middle + 1;
            },
            std::cmp::Ordering::Less => {
                // we look in offset..middle
                end = middle;
                potential_insert = middle;
            },
        }
    }

    potential_insert as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        search_insert(&nums, target)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0035::search_insert;

    #[test]
    fn test() {
        let input = vec![1, 3, 5, 6];

        assert_eq!(search_insert(&input, 5), 2);
    }

    #[test]
    fn test_2() {
        let input = vec![1, 3, 4, 5];

        assert_eq!(search_insert(&input, 2), 1);
    }

    #[test]
    fn test_3() {
        let input = vec![1, 3, 5, 6];

        assert_eq!(search_insert(&input, 7), 4);
    }

    #[test]
    fn test_4() {
        let input = vec![1, 3];

        assert_eq!(search_insert(&input, 1), 0);
    }

    #[test]
    fn test_5() {
        let input = vec![1, 3, 5, 6, 7];

        assert_eq!(search_insert(&input, 8), 5);
    }
}
