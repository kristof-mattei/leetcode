use crate::shared::Solution;

fn search_range(nums: &[i32], target: i32) -> Vec<i32> {
    let mut offset = 0;
    let mut end = nums.len();

    let mut start_index = None;

    while offset < end {
        let middle = (end - offset) / 2 + offset;

        match target.cmp(&nums[middle]) {
            std::cmp::Ordering::Equal => {
                end = middle;
                start_index = Some(middle);
            },
            std::cmp::Ordering::Greater => {
                // we narrow down the scope
                offset = middle + 1;
            },
            std::cmp::Ordering::Less => {
                // we look in offset..middle
                end = middle;
            },
        }
    }

    if let Some(start_index) = start_index {
        offset = start_index;
        end = nums.len();

        let mut end_index = start_index;

        while offset < end {
            let middle = (end - offset) / 2 + offset;
            match target.cmp(&nums[middle]) {
                std::cmp::Ordering::Equal => {
                    offset = middle + 1;
                    end_index = middle;
                },
                std::cmp::Ordering::Greater => {
                    // we narrow down the scope
                    offset = middle + 1;
                },
                std::cmp::Ordering::Less => {
                    // we look in offset..middle
                    end = middle;
                },
            }
        }

        return vec![start_index as i32, end_index as i32];
    }
    return vec![-1, -1];
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        search_range(&nums, target)
    }
}

#[cfg(test)]
mod test {
    use crate::problem_34::search_range;

    #[test]
    fn test() {
        let input = vec![5, 7, 7, 8, 8, 10];

        assert_eq!(search_range(&input, 8), vec![3, 4]);
    }

    #[test]
    fn test_2() {
        let input = vec![5, 7, 7, 8, 8, 10];

        assert_eq!(search_range(&input, 6), vec![-1, -1]);
    }

    #[test]
    fn test_3() {
        let input = vec![];

        assert_eq!(search_range(&input, 0), vec![-1, -1]);
    }
}
