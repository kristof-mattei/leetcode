impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value)]
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        contains_nearby_duplicate(&nums, k)
    }
}

fn contains_nearby_duplicate(nums: &[i32], k: i32) -> bool {
    let k = k as usize;

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len().min(i + 1 + k) {
            if nums[i] == nums[j] && (j - i) <= k {
                return true;
            }
        }
    }

    false
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0219::contains_nearby_duplicate;

    #[test]
    fn test_1() {
        assert!(contains_nearby_duplicate(&[1, 2, 3, 1], 3));
    }

    #[test]
    fn test_2() {
        assert!(contains_nearby_duplicate(&[1, 0, 1, 1], 1));
    }

    #[test]
    fn test_3() {
        assert!(!contains_nearby_duplicate(&[1, 2, 3, 1, 2, 3], 2));
    }

    #[test]
    fn test_4() {
        let nums = [
            313, 64, 612, 515, 412, 661, 244, 164, 492, 744, 233, 579, 62, 786, 342, 817, 838, 396,
            230, 79, 570, 702, 124, 727, 586, 542, 919, 372, 187, 626, 869, 923, 103, 932, 368,
            891, 411, 125, 724, 287, 575, 326, 88, 125, 746, 117, 363, 8, 881, 441, 542, 653, 211,
            180, 620, 175, 747, 276, 832, 772, 165, 733, 574, 186, 778, 586, 601, 165, 422, 956,
            357, 134, 857, 114, 450, 64, 494, 679, 495, 205, 859, 136, 477, 879, 940, 139, 903,
            689, 954, 335, 859, 78, 20,
        ];

        assert!(contains_nearby_duplicate(&nums, 22));
    }
}
