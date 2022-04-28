use std::cmp;

fn area(heights: &[usize]) -> usize {
    let mut seen = Vec::with_capacity(heights.len());
    let mut max_area = 0;

    for (i, &h) in heights.iter().enumerate() {
        while seen.last().map_or(false, |&l| h < heights[l]) {
            let height = heights[seen.pop().unwrap()];

            let start = seen.last().map_or(0, |&l| l + 1);

            let area = (i - start) * height;

            max_area = cmp::max(max_area, area);
        }

        seen.push(i);
    }

    while let Some(i) = seen.pop() {
        let height = heights[i];

        let start = seen.last().map_or(0, |&l| l + 1);

        max_area = cmp::max(max_area, height * (heights.len() - start));
    }

    max_area
}

fn maximal_rectangle(matrix: &[Vec<char>]) -> i32 {
    if matrix.is_empty() {
        return 0;
    }

    let mut max = 0;
    let mut heights = vec![0; matrix[0].len()];

    for row in matrix {
        for (col_index, &c) in row.iter().enumerate() {
            if c == '1' {
                heights[col_index] += 1;
            } else {
                heights[col_index] = 0;
            };
        }

        max = cmp::max(area(&heights), max);
    }

    max as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        maximal_rectangle(&matrix)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0085::maximal_rectangle;

    #[test]
    fn test_1() {
        let input = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];

        assert_eq!(maximal_rectangle(&input), 6);
    }

    #[test]
    fn test_2() {
        let input = vec![vec!['1']];

        assert_eq!(maximal_rectangle(&input), 1);
    }

    #[test]
    fn test_3() {
        let input = vec![vec!['0']];

        assert_eq!(maximal_rectangle(&input), 0);
    }

    #[test]
    fn test_4() {
        let input = vec![vec!['0', '1'], vec!['1', '0']];

        assert_eq!(maximal_rectangle(&input), 1);
    }
}
