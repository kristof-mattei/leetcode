fn rob_helper(nums: &[i32]) -> i32 {
    let mut neighbor_2_to_left_acc = nums[0];
    let mut neighbor_to_left_or_current_acc = nums[1].max(nums[0]);

    for current_house_value in nums.iter().skip(2) {
        let potential_new_value = neighbor_2_to_left_acc + current_house_value;

        if potential_new_value > neighbor_to_left_or_current_acc {
            // shift
            neighbor_2_to_left_acc = neighbor_to_left_or_current_acc;
            neighbor_to_left_or_current_acc = potential_new_value;
        } else {
            // we didn't chose the current house, so we need
            // to set both values to our left neighbor
            // to ensure next iteration is correct
            neighbor_2_to_left_acc = neighbor_to_left_or_current_acc;
        }
    }

    neighbor_2_to_left_acc.max(neighbor_to_left_or_current_acc)
}

fn rob(nums: &[i32]) -> i32 {
    let len = nums.len();

    if len == 0 {
        return 0;
    } else if len == 1 {
        return nums[0];
    } else if len == 2 {
        return i32::max(nums[0], nums[1]);
    }

    i32::max(
        rob_helper(&nums[..(nums.len() - 1)]),
        rob_helper(&nums[1..]),
    )
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        rob(&nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0213::rob;

    #[test]
    fn test_1() {
        assert_eq!(rob(&[2, 3, 2]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(rob(&[1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(rob(&[1, 2, 3]), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(rob(&[0, 1, 0, 0, 0, 1000]), 1001);
    }

    #[test]
    fn test_5() {
        assert_eq!(rob(&[0, 0]), 0);
    }
}
