impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        majority_element(&nums)
    }
}

fn majority_element(nums: &[i32]) -> Vec<i32> {
    let mut value_1 = i32::MIN;
    let mut value_2 = i32::MIN;

    let mut value_1_count = 0;
    let mut value_2_count = 0;

    let one_third_length = nums.len() / 3;

    for num in nums {
        if value_1 == *num {
            value_1_count += 1;
        } else if value_2 == *num {
            value_2_count += 1;
        } else if value_1_count == 0 {
            value_1 = *num;
            value_1_count = 1;
        } else if value_2_count == 0 {
            value_2 = *num;
            value_2_count = 1;
        } else {
            value_1_count -= 1;
            value_2_count -= 1;
        }
    }

    let mut value_1_count = 0;
    let mut value_2_count = 0;

    for v in nums {
        if *v == value_1 {
            value_1_count += 1;
        } else if *v == value_2 {
            value_2_count += 1;
        } else {
            // ...
        }
    }

    [
        (value_1_count > one_third_length).then_some(value_1),
        (value_2_count > one_third_length).then_some(value_2),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0228::majority_element;

    #[test]
    fn test_1() {
        assert_eq!(majority_element(&[3, 2, 3]), [3]);
    }

    #[test]
    fn test_2() {
        assert_eq!(majority_element(&[1]), [1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(majority_element(&[1, 2]), [1, 2]);
    }
}
