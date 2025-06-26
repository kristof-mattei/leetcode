impl Solution {
    #[expect(non_snake_case)]
    fn isBadVersion(&self, v: i32) -> bool {
        v >= self.bad
    }

    #[must_use]
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut low = 1;
        let mut high = n;

        while low < high {
            // find mid point between low and high
            let mid = low + ((high - low) / 2);

            if self.isBadVersion(mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low
    }
}

pub struct Solution {
    bad: i32,
}

#[cfg(test)]
mod tests {
    use crate::problem_0278::Solution;

    #[test]
    fn test_1() {
        let s = Solution { bad: 4 };

        assert_eq!(s.first_bad_version(5), 4);
    }
}
