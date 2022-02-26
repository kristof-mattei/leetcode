use crate::shared::Solution;

fn next_permutation(nums: &mut Vec<i32>) {
    let mut max_index = 0;

    // find the first index of a number where the number before it is lower than the current
    for i in (1..nums.len()).rev() {
        if nums[i] > nums[i - 1] {
            max_index = i;
            break;
        }
    }

    if max_index != 0 {
        // from the end backwards we find the first value that is strictly larger than
        // than the value just before it
        let minimum = nums[max_index - 1];

        // we have to do this from the right, otherwise
        // we can't reverse, but we have to sort
        // e.g.:
        // 2,5,4,3,3,2,1
        // --^---^-^----
        // Highlighted: max & the 3 & 3 that would be candidates to
        // replace the 2 in front of the max
        // if we search from left we'd find the first 3, and replace that with the 2:
        // 3,5,4,2,3,2,1
        // reversing that doesn't work.
        // but if we search from the right we get
        // 3,5,4,3,2,2,1
        // and then we CAN just reverse the part after max without sorting

        for i in (max_index..nums.len()).rev() {
            if nums[i] > minimum {
                nums.swap(i, max_index - 1);
                break;
            }
        }
        // for i in (max_index..len).rev() {
        //     if nums[i] > minimum {
        //         nums.swap(i, max_index - 1);
        //         break;
        //     }
        // }
    }

    // we bumped the number in front of max_index
    // so all the numbers after it now need to be sorted
    // but since they are already sorted because
    // they are descending from the right, we can just reverse
    nums[max_index..].reverse();
}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        next_permutation(nums);
    }
}

#[cfg(test)]
mod test {
    use crate::problem_31::next_permutation;

    #[test]
    fn test() {
        let mut input = vec![1, 2, 3];
        next_permutation(&mut input);
        assert_eq!(input, vec![1, 3, 2]);
    }

    #[test]
    fn test_2() {
        let mut input = vec![3, 2, 1];
        next_permutation(&mut input);
        assert_eq!(input, vec![1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let mut input = vec![1, 3, 2];
        next_permutation(&mut input);
        assert_eq!(input, vec![2, 1, 3]);
    }

    #[test]
    fn test_4() {
        let mut input = vec![1, 1, 5];
        next_permutation(&mut input);
        assert_eq!(input, vec![1, 5, 1]);
    }

    #[test]
    fn test_5() {
        let mut input = vec![2, 3, 1];
        next_permutation(&mut input);
        assert_eq!(input, vec![3, 1, 2]);
    }

    #[test]
    fn test_6() {
        let mut input = vec![5, 4, 7, 5, 3, 2];
        next_permutation(&mut input);
        assert_eq!(input, vec![5, 5, 2, 3, 4, 7]);
    }

    #[test]
    fn test_7() {
        let mut input = vec![5, 4, 7, 3, 6, 5, 2];
        next_permutation(&mut input);
        assert_eq!(input, vec![5, 4, 7, 5, 2, 3, 6]);
    }

    #[test]
    fn test_8() {
        let mut input = vec![3, 1, 2];
        next_permutation(&mut input);
        assert_eq!(input, vec![3, 2, 1]);
    }

    #[test]
    fn test_9() {
        let mut input = vec![3, 1, 2];
        next_permutation(&mut input);
        assert_eq!(input, vec![3, 2, 1]);
    }

    #[test]
    fn test_10() {
        let mut input = vec![2, 2, 7, 5, 4, 3, 2, 2, 1];
        next_permutation(&mut input);
        assert_eq!(input, vec![2, 3, 1, 2, 2, 2, 4, 5, 7]);
    }

    #[test]
    fn test_11() {
        let mut input = vec![9, 8, 7, 10, 2];
        next_permutation(&mut input);
        assert_eq!(input, vec![9, 8, 10, 2, 7]);
    }

    #[test]
    fn test_12() {
        let mut input = vec![8, 9, 13, 11, 10, 10, 10, 9, 8, 7];
        next_permutation(&mut input);
        assert_eq!(input, vec![8, 10, 7, 8, 9, 9, 10, 10, 11, 13]);
    }
}
