struct NumArray {
    nums: Vec<i32>,
    prefix_sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let prefix_sums =
            nums.iter()
                .enumerate()
                .fold(vec![0; nums.len() + 1], |mut ps, (index, value)| {
                    ps[index + 1] = ps[index] + value;
                    ps
                })[1..]
                .to_vec();

        Self { nums, prefix_sums }
    }

    #[cfg_attr(not(test), expect(unused))]
    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;

        let original = self.nums[index];

        let diff = original - val;

        self.nums[index] = val;

        for i in index..self.prefix_sums.len() {
            self.prefix_sums[i] += -diff;
        }
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
    let num_array = NumArray::new(input.to_vec());

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
    use crate::problem_0307::{NumArray, sum_ranges};

    #[test]
    fn test_1() {
        assert_eq!(
            sum_ranges(&[-2, 0, 3, -5, 2, -1], &[(0, 2), (2, 5), (0, 5)]),
            &[1, -1, -3]
        );
    }

    #[test]
    fn test_2() {
        let mut num_array = NumArray::new(vec![-1]);
        assert_eq!(num_array.sum_range(0, 0), -1);
        num_array.update(0, 1);
        assert_eq!(num_array.sum_range(0, 0), 1);
    }

    #[test]
    fn test_3() {
        let mut num_array = NumArray::new(vec![7, 2, 7, 2, 0]);

        // [update",
        num_array.update(4, 6);
        //"update",
        num_array.update(0, 2);
        //"update",
        num_array.update(0, 9);
        //"sumRange",
        assert_eq!(num_array.sum_range(4, 4), 6);
        //"update",
        num_array.update(3, 8);
        //"sumRange",
        assert_eq!(num_array.sum_range(0, 4), 32);
        //"update",
        num_array.update(4, 1);
        //"sumRange",
        assert_eq!(num_array.sum_range(0, 3), 26);
        //"sumRange",
        assert_eq!(num_array.sum_range(0, 4), 27);
        //"update"]
        num_array.update(0, 4);
    }
}
