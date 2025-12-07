impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        single_number(&nums)
    }
}

fn single_number(nums: &[i32]) -> Vec<i32> {
    let xor = nums.iter().fold(0, |mut acc, current| {
        acc ^= current;
        acc
    });

    let set_bit = xor & (!xor + 1);

    let mut a = 0;
    let mut b = 0;

    for num in nums {
        if num & set_bit == 0 {
            a ^= num;
        } else {
            b ^= num;
        }
    }

    vec![a, b]
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use hashbrown::HashSet;

    use crate::problem_0260::single_number;

    #[test]
    fn test_1() {
        let result = HashSet::<_>::from_iter(single_number(&[1, 2, 1, 3, 2, 5]));

        let expected = HashSet::from_iter([3, 5]);

        assert_eq!(result, expected);
    }
}
