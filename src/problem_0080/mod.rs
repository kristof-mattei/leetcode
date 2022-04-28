fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut index = 2;

    while let Some(&n) = nums.get(index) {
        if n == nums[index - 1] && n == nums[index - 2] {
            nums.remove(index);
        } else {
            index += 1;
        }
    }

    nums.len() as i32
}

impl Solution {
    #[must_use]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        remove_duplicates(nums)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0080::remove_duplicates;

    #[test]
    fn test_1() {
        let mut input = vec![1, 1, 1, 2, 2, 3];

        let r = remove_duplicates(&mut input);

        assert_eq!(r, 5);
        assert_eq!(input, [1, 1, 2, 2, 3]);
    }
}
