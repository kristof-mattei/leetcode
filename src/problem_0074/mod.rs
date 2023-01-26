enum RangeResult {
    Exact(usize),
    Close(usize),
}

fn find_range(nums: &[i32], target: i32) -> RangeResult {
    let mut offset = 0;
    let mut end = nums.len();

    let mut index = 0;

    while offset < end {
        let middle = (end - offset) / 2 + offset;
        match target.cmp(&nums[middle]) {
            std::cmp::Ordering::Equal => {
                return RangeResult::Exact(middle);
            },
            std::cmp::Ordering::Greater => {
                // we narrow down the scope
                offset = middle + 1;
                index = middle + 1;
            },
            std::cmp::Ordering::Less => {
                // we look in offset..middle
                end = middle;
                index = middle;
            },
        }
    }

    RangeResult::Close(index)
}

fn search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    let row_search_result = find_range(&matrix.iter().map(|r| r[0]).collect::<Vec<_>>(), target);
    match row_search_result {
        RangeResult::Exact(_) => true,
        RangeResult::Close(r) if r > 0 => {
            matches!(find_range(&matrix[r - 1], target), RangeResult::Exact(_))
        },
        RangeResult::Close(_) => false,
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        search_matrix(&matrix, target)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0074::search_matrix;

    #[test]
    fn test_1() {
        let input = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];

        assert!(search_matrix(&input, 3));
    }

    #[test]
    fn test_2() {
        let input = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];

        assert!(!search_matrix(&input, 13));
    }

    #[test]
    fn test_3() {
        let input = vec![
            vec![-10, -5, -3, -2],
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60],
        ];

        assert!(search_matrix(&input, 3));
    }

    #[test]
    fn test_4() {
        let input = vec![vec![1]];

        assert!(!search_matrix(&input, 0));
    }
}
