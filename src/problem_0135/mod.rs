use crate::shared::Solution;

// fn is_higher(ratings: &[i32], index: usize) -> bool {
//     if index + 1 == ratings.len() {
//         return ratings[index - 1] < ratings[index];
//     }

//     if index == 0 {
//         return ratings[index] > ratings[index + 1];
//     }

//     return ratings[index - 1] < ratings[index] || ratings[index] > ratings[index + 1];
// }

fn candy(ratings: &[i32]) -> i32 {
    // let result = vec![1; ratings.len()];

    // let mut change_made = true;

    // while change_made {
    //     change_made = false;

    //     for i in 0..ratings.len() {
    //         if is_higher(ratings, i) {
    //             change_made = true;
    //             bump_result_until_higher(&mut result, i);
    //         }
    //     }
    // }

    0
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn candy(ratings: Vec<i32>) -> i32 {
        candy(&ratings)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0135::candy;

    #[test]
    fn test_1() {
        assert_eq!(candy(&[1, 0, 2]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(candy(&[1, 2, 2]), 4);
    }

    #[test]
    fn test_3() {
        // assert_eq!(candy(&[1, 2, 3, 4, 5]), 5);
    }
}
