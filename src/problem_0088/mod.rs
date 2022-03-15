use crate::shared::Solution;

fn merge(nums1: &mut Vec<i32>, m: usize, nums2: &mut Vec<i32>, n: usize) {
    (&mut nums1[m..m + n]).copy_from_slice(nums2);

    for i in m..m + n {
        let mut j = i;
        while j > 0 {
            if nums1[j] < nums1[j - 1] {
                nums1.swap(j, j - 1);
                j -= 1;
                continue;
            }
            if j == i {
                return;
            }
            break;
        }
    }
}

impl Solution {
    pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        merge(nums1, m as usize, nums2, n as usize);
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0088::merge;

    #[test]
    fn test_1() {
        let mut input1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut input2 = vec![2, 5, 6];
        let n = 3;

        merge(&mut input1, m, &mut input2, n);

        assert_eq!(input1, [1, 2, 2, 3, 5, 6]);
    }
    #[test]
    fn test_2() {
        let mut input1 = vec![1];
        let m = 1;
        let mut input2 = vec![];
        let n = 0;

        merge(&mut input1, m, &mut input2, n);

        assert_eq!(input1, [1]);
    }
    #[test]
    fn test_3() {
        let mut input1 = vec![0];
        let m = 0;
        let mut input2 = vec![1];
        let n = 1;

        merge(&mut input1, m, &mut input2, n);

        assert_eq!(input1, [1]);
    }
}
