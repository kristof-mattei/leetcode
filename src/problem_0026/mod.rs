fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut previous = i32::MIN;
    let mut index: usize = 0;
    let mut length = nums.len();

    while index < length {
        let n = nums[index];

        match n.cmp(&previous) {
            std::cmp::Ordering::Equal => {
                let _ = nums.remove(index);
                length -= 1;
            },
            std::cmp::Ordering::Greater => {
                previous = n;
                index += 1;
            },
            std::cmp::Ordering::Less => unreachable!(),
        };
    }
    (length) as i32
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        remove_duplicates(nums)

        // or
        // nums.dedup();

        // nums.len() as i32
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0026::remove_duplicates;

    #[test]
    fn test() {
        let mut input = vec![1, 1, 2];

        let result = remove_duplicates(&mut input);

        assert_eq!(result, 2);
        assert_eq!(input, vec![1, 2]);
    }

    #[test]
    fn test_2() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        let result = remove_duplicates(&mut input);

        assert_eq!(result, 5);
        assert_eq!(input, vec![0, 1, 2, 3, 4]);
    }
}
