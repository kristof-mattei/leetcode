struct NumArray {
    prefix_sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: &[i32]) -> Self {
        let prefix_sums =
            nums.iter()
                .enumerate()
                .fold(vec![0; nums.len() + 1], |mut ps, (index, value)| {
                    ps[index + 1] = ps[index] + value;
                    ps
                })[1..]
                .to_vec();

        Self { prefix_sums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.prefix_sums[right as usize]
        } else {
            self.prefix_sums[right as usize] - self.prefix_sums[(left - 1) as usize]
        }
    }
}

fn sum_ranges(input: &[i32], regions: &[(i32, i32)]) -> Vec<i32> {
    let num_array = NumArray::new(input);

    let mut result = Vec::with_capacity(regions.len());

    for &(left, right) in regions {
        let sum = num_array.sum_range(left, right);

        result.push(sum);
    }

    result
}

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn sum_range(nums: Vec<i32>, regions: Vec<(i32, i32)>) -> Vec<i32> {
        sum_ranges(&nums, &regions)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0303::sum_ranges;

    #[test]
    fn test_1() {
        assert_eq!(
            sum_ranges(&[-2, 0, 3, -5, 2, -1], &[(0, 2), (2, 5), (0, 5)]),
            &[1, -1, -3]
        );
    }
}
