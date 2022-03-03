use crate::shared::Solution;

fn trap(height: &[i32]) -> i32 {
    let mut trapped = 0;
    let len = height.len();
    let (max_height_index, _) = height.iter().enumerate().max_by_key(|(_, v)| *v).unwrap();

    let mut max_height_2 = height[0];
    for h in height.iter().take(max_height_index).skip(1) {
        if h > &max_height_2 {
            max_height_2 = *h;
        } else {
            trapped += max_height_2 - h;
        }
    }

    max_height_2 = height[len - 1];
    for h in height.iter().rev().take(len - max_height_index - 1).skip(1) {
        if h > &max_height_2 {
            max_height_2 = *h;
        } else {
            trapped += max_height_2 - h;
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

#[cfg(test)]
mod tests {
    use crate::problem_42::trap;

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
