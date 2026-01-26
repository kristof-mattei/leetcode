fn largest_rectangle_area(heights: &[i32]) -> i32 {
    let mut stack = Vec::with_capacity(heights.len() + 1);
    stack.push((-1_i32, 0_i32)); // sentinel

    let mut max = 0;

    for (i, &height) in heights.iter().enumerate() {
        while stack.len() > 1
            && let Some((_, prev_height)) = stack.pop_if(|&mut (_, p): &mut _| p > height)
        {
            let last_index = stack.last().unwrap().0;
            max = i32::max(max, prev_height * ((i as i32) - last_index - 1));
        }

        stack.push((i as i32, height));
    }

    let length = heights.len();

    while stack.len() > 1 {
        let (_, prev_height) = stack.pop().unwrap();

        let last_index = stack.last().unwrap().0;

        max = i32::max(max, prev_height * (length as i32 - last_index - 1));
    }

    max
}

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        largest_rectangle_area(&heights)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0084::largest_rectangle_area;

    #[test]
    fn test_1() {
        assert_eq!(largest_rectangle_area(&[2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(largest_rectangle_area(&[2, 4]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(largest_rectangle_area(&[1]), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(largest_rectangle_area(&[4, 2, 0, 3, 2, 4, 3, 4]), 10);
    }
}
