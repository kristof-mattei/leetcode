fn largest_number(mut nums: Vec<i32>) -> String {
    nums.sort_by(|l, r| format!("{}{}", r, l).cmp(&format!("{}{}", l, r)));

    // if the biggest number is a 0, then all is zero
    if nums[0] == 0 {
        return String::from("0");
    }

    nums.into_iter().map(|n| n.to_string()).collect::<String>()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn largest_number(nums: Vec<i32>) -> String {
        largest_number(nums)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0179::largest_number;

    #[test]
    fn test_1() {
        assert_eq!(largest_number([10, 2].to_vec()), "210");
    }

    #[test]
    fn test_2() {
        assert_eq!(largest_number([3, 30, 34, 5, 9].to_vec()), "9534330");
    }

    #[test]
    fn test_3() {
        assert_eq!(largest_number([9, 12, 4, 0].to_vec()), "94120");
    }
}
