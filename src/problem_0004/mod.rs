fn find_median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
    let len = nums1.len() + nums2.len();

    let mut nums1_index = 0;
    let mut nums2_index = 0;

    let mut last = 0;
    let mut last_before_last = 0;

    for _ in 0..=(len / 2) {
        let n1 = nums1.get(nums1_index);
        let n2 = nums2.get(nums2_index);

        let current = match (n1, n2) {
            (None, None) => unreachable!(),
            (None, Some(&v)) => {
                nums2_index += 1;
                v
            },
            (Some(&v), None) => {
                nums1_index += 1;
                v
            },

            (Some(&l), Some(&r)) => {
                if l < r {
                    nums1_index += 1;
                    l
                } else {
                    nums2_index += 1;
                    r
                }
            },
        };

        last_before_last = last;
        last = current;
    }

    if len % 2 == 0 {
        (f64::from(last_before_last) + f64::from(last)) / 2f64
    } else {
        f64::from(last)
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        find_median_sorted_arrays(&nums1, &nums2)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::{problem_0004::find_median_sorted_arrays, utils::nearly_equal};

    #[test]
    fn test_1() {
        assert!(nearly_equal(
            find_median_sorted_arrays(&[1, 3], &[2]),
            2f64,
            f64::EPSILON
        ));
    }

    #[test]
    fn test_2() {
        assert!(nearly_equal(
            find_median_sorted_arrays(&[1, 2], &[3, 4]),
            2.5f64,
            f64::EPSILON
        ));
    }

    #[test]
    fn test_3() {
        assert!(nearly_equal(
            find_median_sorted_arrays(&[0, 0], &[0, 0]),
            0f64,
            f64::EPSILON
        ));
    }
}
