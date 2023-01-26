fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
    heights.push(0);

    let mut stack = vec![(0, heights[0])];
    let mut max = heights[0];

    for (i, height) in heights.into_iter().enumerate().skip(1) {
        while let Some(prev_height) = stack.last().map(|(_, p)| *p) {
            if prev_height <= height {
                break;
            }

            stack.pop();

            let last_idx = stack.last().map_or(-1, |(i, _)| *i);

            max = i32::max(max, prev_height * ((i as i32) - last_idx - 1));
        }

        stack.push((i as i32, height));
    }

    max
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        largest_rectangle_area(heights)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0084::largest_rectangle_area;

    #[test]
    fn test_1() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(largest_rectangle_area(vec![2, 4]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(largest_rectangle_area(vec![1]), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(largest_rectangle_area(vec![4, 2, 0, 3, 2, 4, 3, 4]), 10);
    }
}
