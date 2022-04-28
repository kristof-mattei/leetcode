fn trap(height: &[i32]) -> i32 {
    let mut index_l = 0;
    let mut index_r = height.len() - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut trapped = 0;

    while index_l < index_r {
        if height[index_l] < height[index_r] {
            if height[index_l] >= left_max {
                left_max = height[index_l];
            } else {
                trapped += left_max - height[index_l];
            }
            index_l += 1;
        } else {
            if height[index_r] >= right_max {
                right_max = height[index_r];
            } else {
                trapped += right_max - height[index_r];
            }
            index_r -= 1;
        }
    }
    trapped
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn trap(height: Vec<i32>) -> i32 {
        trap(&height)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0042::trap;

    #[test]
    fn test() {
        assert_eq!(trap(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(trap(&[4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn test_3() {
        assert_eq!(trap(&[4, 2, 3]), 1);
    }

    // #[test]
    // fn test_4() {
    //     let p = include_str!("inputs/huge.txt").to_string();

    //     p.

    // }
}
