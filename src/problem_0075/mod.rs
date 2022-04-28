fn sort_colors(nums: &mut [i32]) {
    let mut arr: [usize; 3] = [0; 3];

    for n in nums.iter() {
        arr[*n as usize] += 1;
    }

    for color in nums.iter_mut().take(arr[0]) {
        *color = 0;
    }

    let mut offset = arr[0];

    for color in nums.iter_mut().skip(offset).take(arr[1]) {
        *color = 1;
    }

    offset += arr[1];

    for color in nums.iter_mut().skip(offset).take(arr[2]) {
        *color = 2;
    }
}

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        sort_colors(nums);
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0075::sort_colors;

    #[test]
    fn test_1() {
        let mut input = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut input);
        assert_eq!(input, [0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_2() {
        let mut input = vec![2, 0, 1];
        sort_colors(&mut input);
        assert_eq!(input, [0, 1, 2]);
    }
}
