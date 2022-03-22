use crate::shared::Solution;

fn min_domino_rotations(tops: &[i32], bottoms: &[i32]) -> i32 {
    'outer: for domino in 1..=6 {
        let mut top_count = 0;
        let mut bottom_count = 0;

        for (&top_d, &bottom_d) in tops.iter().zip(bottoms.iter()) {
            match (top_d == domino, bottom_d == domino) {
                (true, true) => {
                    // we don't need to count this position
                },
                (true, false) => {
                    top_count += 1;
                },
                (false, true) => {
                    bottom_count += 1;
                },
                (false, false) => {
                    // we hit a combination where we don't have `domino`, so we can't make it work.
                    continue 'outer;
                },
            }
        }

        return i32::min(top_count, bottom_count);
    }
    -1
}

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        min_domino_rotations(&tops, &bottoms)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_1007::min_domino_rotations;

    #[test]
    fn test_1() {
        assert_eq!(
            min_domino_rotations(&[2, 1, 2, 4, 2, 2], &[5, 2, 6, 2, 3, 2]),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(min_domino_rotations(&[3, 5, 1, 2, 3], &[3, 6, 3, 3, 4]), -1);
    }
}
