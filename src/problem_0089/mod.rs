use crate::shared::Solution;

fn gray_code(n: i32) -> Vec<i32> {
    let length = 2usize.pow(n as u32) as usize;
    let mut result = vec![0; length];

    #[allow(clippy::needless_range_loop)]
    for i in 0..length {
        result[i] = (i ^ (i >> 1)) as i32;
    }

    result
}

impl Solution {
    #[must_use]
    pub fn gray_code(n: i32) -> Vec<i32> {
        gray_code(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0089::gray_code;

    #[test]
    fn test_1() {
        let input = 7;

        let result = gray_code(input);

        assert_eq!(
            result,
            [
                0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8, 24, 25, 27, 26, 30, 31, 29,
                28, 20, 21, 23, 22, 18, 19, 17, 16, 48, 49, 51, 50, 54, 55, 53, 52, 60, 61, 63, 62,
                58, 59, 57, 56, 40, 41, 43, 42, 46, 47, 45, 44, 36, 37, 39, 38, 34, 35, 33, 32, 96,
                97, 99, 98, 102, 103, 101, 100, 108, 109, 111, 110, 106, 107, 105, 104, 120, 121,
                123, 122, 126, 127, 125, 124, 116, 117, 119, 118, 114, 115, 113, 112, 80, 81, 83,
                82, 86, 87, 85, 84, 92, 93, 95, 94, 90, 91, 89, 88, 72, 73, 75, 74, 78, 79, 77, 76,
                68, 69, 71, 70, 66, 67, 65, 64
            ]
        );
    }
}
