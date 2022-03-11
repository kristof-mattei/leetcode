use crate::shared::Solution;

#[inline]
fn min3<T>(f: T, s: T, t: T) -> T
where
    T: std::cmp::Ord,
{
    f.min(s.min(t))
}

// #[inline]
// #[allow(clippy::needless_range_loop)]
// fn min_distance(word1: &str, word2: &str) -> i32 {
//     let word1_len = word1.chars().count();
//     let word2_len = word2.chars().count();

//     let mut matrix: Vec<Vec<usize>> = vec![vec![0; word2_len + 1]; word1_len + 1];

//     for i in 1..=word1_len {
//         matrix[i][0] = i;
//     }
//     for i in 1..=word2_len {
//         matrix[0][i] = i;
//     }

//     // apply edit operations
//     for (i, w1_c) in word1.chars().enumerate() {
//         for (j, w2_c) in word2.chars().enumerate() {
//             let substitution = if w1_c == w2_c { 0 } else { 1 };

//             matrix[i + 1][j + 1] = min3(
//                 matrix[i][j + 1] + 1,        // deletion
//                 matrix[i + 1][j] + 1,        // insertion
//                 matrix[i][j] + substitution, // substitution
//             );
//         }
//     }

//     matrix[word1_len][word2_len] as i32
// }

#[inline]
#[allow(clippy::range_plus_one)]
fn min_distance(word1: &str, word2: &str) -> i32 {
    let word1_char_len = word1.chars().count();
    let word2_char_len = word2.chars().count();

    if word1 == word2 {
        return 0;
    }

    if word1_char_len == 0 {
        return word2_char_len as i32;
    }

    if word2_char_len == 0 {
        return word1_char_len as i32;
    }

    let mut v0 = (0..(word2_char_len + 1)).collect::<Vec<_>>();
    let mut v1 = vec![0; word2_char_len + 1];

    for (i, s_char) in word1.chars().enumerate() {
        v1[0] = i + 1;

        for (j, t_char) in word2.chars().enumerate() {
            let deletion_cost = v0[j + 1] + 1;
            let insertion_cost = v1[j] + 1;
            let substitution_cost = if s_char == t_char { v0[j] } else { v0[j] + 1 };

            v1[j + 1] = min3(deletion_cost, insertion_cost, substitution_cost);
        }

        std::mem::swap(&mut v0, &mut v1);
    }

    v0[word2_char_len] as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_distance(word1: String, word2: String) -> i32 {
        min_distance(&word1, &word2)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0072::min_distance;

    #[test]
    fn test_1() {
        assert_eq!(min_distance("horse", "ros"), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(min_distance("intention", "execution"), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(min_distance("a", "b"), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(min_distance("a", "ab"), 1);
    }

    #[test]
    fn test_5() {
        assert_eq!(min_distance("park", "spake"), 3);
    }
}
