use crate::shared::Solution;

fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() < 3 {
        return 0;
    }

    nums.sort_unstable();

    let mut shortest_distance_from_target = i32::MAX;
    let mut sum_shortest_distance_from_target = i32::MAX;

    for tripple_1_index in 0..nums.len() - 2 {
        if tripple_1_index > 0 && nums[tripple_1_index] == nums[tripple_1_index - 1] {
            continue;
        }

        let mut tripple_2_index = tripple_1_index + 1;
        let mut tripple_3_index = nums.len() - 1;

        while tripple_2_index < tripple_3_index {
            let sum = nums[tripple_1_index] + nums[tripple_2_index] + nums[tripple_3_index];

            match sum {
                _ if sum == target => {
                    return sum;
                },
                _ if sum > target => {
                    // Decrement to reduce sum value (we're sorted)
                    tripple_3_index -= 1;
                },
                _ => {
                    // Increment to increase sum value
                    tripple_2_index += 1;
                },
            }

            let distance_from_target = (target - sum).abs();
            if distance_from_target < shortest_distance_from_target {
                shortest_distance_from_target = distance_from_target;
                sum_shortest_distance_from_target = sum;
            }
        }
    }
    sum_shortest_distance_from_target
}

impl Solution {
    #[must_use]
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        three_sum_closest(nums, target)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_16::three_sum_closest;

    #[test]
    fn test() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
}
