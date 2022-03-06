use std::vec::Vec;

use crate::shared::Solution;

fn spiral_order(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut results = Vec::new();
    let width = matrix.get(0).map_or(0, Vec::len) - 1;
    let height = matrix.len() - 1;
    let mut offset = 0;

    loop {
        // go right
        for &v in &matrix[offset][offset..=(width - offset)] {
            results.push(v);
        }

        if offset == height - offset {
            break;
        }

        // go to bottom
        for v in matrix[(offset + 1)..=(height - offset)]
            .iter()
            .map(|v| v[width - offset])
        {
            results.push(v);
        }

        if offset == width - offset {
            break;
        }

        // go left
        #[allow(clippy::range_minus_one)]
        for &v in matrix[height - offset][offset..=(width - offset - 1)]
            .iter()
            .rev()
        {
            results.push(v);
        }

        if offset == height - offset - 1 {
            break;
        }

        // go up
        #[allow(clippy::range_minus_one)]
        for v in matrix[(offset + 1)..=(height - offset - 1)]
            .iter()
            .map(|v| v[offset])
            .rev()
        {
            results.push(v);
        }

        if offset == width - offset - 1 {
            break;
        }

        offset += 1;
    }

    results
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        spiral_order(&matrix)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_54::spiral_order;

    #[test]
    fn test_1() {
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];

        let result = spiral_order(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];

        let result = spiral_order(&[vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let expected = vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10];

        let result = spiral_order(&[
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_4() {
        let expected = vec![1, 2, 3, 6, 9, 12, 11, 10, 7, 4, 5, 8];

        let result = spiral_order(&[
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_5() {
        let expected = vec![2, 5, 8, -1, 0, 4];

        let result = spiral_order(&[vec![2, 5, 8], vec![4, 0, -1]]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_6() {
        let expected = vec![1, 2, 4, 6, 5, 3];

        let result = spiral_order(&[vec![1, 2], vec![3, 4], vec![5, 6]]);

        assert_eq!(result, expected);
    }
}
