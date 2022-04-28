fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut length = nums.len();

    let mut i = 0;

    while i < length {
        if nums[i] == val {
            nums.swap_remove(i);
            length -= 1;
            continue;
        }

        i += 1;
    }

    length as i32
}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        remove_element(nums, val)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0027::remove_element;

    #[test]
    fn test() {
        let mut input = vec![3, 2, 2, 3];

        let result = remove_element(&mut input, 3);

        assert_eq!(result, 2);
        assert_eq!(input, vec![2, 2]);
    }

    #[test]
    fn test_2() {
        let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];

        let result = remove_element(&mut input, 2);

        assert_eq!(result, 5);
        assert_eq!(input, vec![0, 1, 4, 0, 3]);
    }
}
