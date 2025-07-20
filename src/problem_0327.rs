impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        count_range_sum(&nums, lower, upper)
    }
}

struct FenwickTree {
    bit: Vec<i32>,
    n: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self { bit: vec![0; n], n }
    }

    fn sum(&self, mut r: i32) -> i32 {
        let mut result = 0;

        while r >= 0 {
            result += self.bit[r as usize];

            r = (r & (r + 1)) - 1;
        }

        result
    }

    fn add(&mut self, mut index: usize, delta: i32) {
        while index < self.n {
            self.bit[index] += delta;
            index = index | (index + 1);
        }
    }
}

fn count_range_sum(nums: &[i32], lower: i32, upper: i32) -> i32 {
    let mut cache = vec![];

    let n = nums.len();

    let mut sum = 0_i64;

    let mut result = 0;

    for (i, &num) in nums.iter().enumerate() {
        sum += i64::from(num);
        cache.push((sum, i));
    }

    cache.sort_unstable();

    let mut fenwick_tree = FenwickTree::new(n);

    let (mut l, mut r) = (0, 0);

    for i in 0..n {
        while r < n && (cache[i].0 - cache[r].0) >= i64::from(lower) {
            fenwick_tree.add(cache[r].1, 1);
            r += 1;
        }
        while l < n && (cache[i].0 - cache[l].0) > i64::from(upper) {
            fenwick_tree.add(cache[l].1, -1);
            l += 1;
        }

        result += fenwick_tree.sum(cache[i].1 as i32 - 1);

        if cache[i].0 >= i64::from(lower) && cache[i].0 <= i64::from(upper) {
            result += 1;
        }
    }

    result
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0327::count_range_sum;

    #[test]
    fn test_1() {
        assert_eq!(count_range_sum(&[-2, 5, -1], -2, 2), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            count_range_sum(
                &[-2_147_483_647, 0, -2_147_483_647, 2_147_483_647],
                -564,
                3864
            ),
            3
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(count_range_sum(&[0, 0], 0, 0), 3);
    }
}
