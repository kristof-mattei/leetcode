struct NumMatrix {
    prefix_sums: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        let columns = matrix.first().map_or(0, std::vec::Vec::len);

        let mut prefix_sums = vec![vec![0; columns + 1]; rows + 1];

        for row in 1..=rows {
            for column in 1..=columns {
                prefix_sums[row][column] = matrix[row - 1][column - 1]
                    + prefix_sums[row - 1][column]
                    + prefix_sums[row][column - 1]
                    - prefix_sums[row - 1][column - 1];
            }
        }

        Self { prefix_sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;

        (&self.prefix_sums)[row2 + 1][col2 + 1]
            - (&self.prefix_sums)[row1][col2 + 1]
            - (&self.prefix_sums)[row2 + 1][col1]
            + (&self.prefix_sums)[row1][col1]
    }
}

fn sum_regions(input: &[&[i32]], regions: &[(i32, i32, i32, i32)]) -> Vec<i32> {
    let num_matrix = NumMatrix::new(input.iter().map(|row| row.to_vec()).collect());

    let mut result = Vec::with_capacity(regions.len());

    for &(row1, col1, row2, col2) in regions {
        let sum = num_matrix.sum_region(row1, col1, row2, col2);

        result.push(sum);
    }

    result
}

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn sum_regions(matrix: Vec<Vec<i32>>, regions: Vec<(i32, i32, i32, i32)>) -> Vec<i32> {
        sum_regions(&matrix.iter().map(|m| &**m).collect::<Vec<_>>(), &regions)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0304::sum_regions;

    #[test]
    fn test_1() {
        assert_eq!(
            sum_regions(
                &[
                    &[3, 0, 1, 4, 2],
                    &[5, 6, 3, 2, 1],
                    &[1, 2, 0, 1, 5],
                    &[4, 1, 0, 1, 7],
                    &[1, 0, 3, 0, 5]
                ],
                &[(2, 1, 4, 3), (1, 1, 2, 2), (1, 2, 2, 4)]
            ),
            &[8, 11, 12]
        );
    }
}
