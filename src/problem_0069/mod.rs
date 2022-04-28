// use std::cmp::Ordering;

fn my_sqrt(x: i32) -> i32 {
    // if x <= 1 {
    //     return x;
    // }

    // let mut low = 0;
    // let mut high = x / 2 + 1;

    // let mut result = 0;

    // while low <= high {
    //     let mid = low + (high - low) / 2;
    //     match mid.cmp(&(x / mid)) {
    //         Ordering::Greater => high = mid - 1,
    //         Ordering::Less => {
    //             low = mid + 1;
    //             result = mid;
    //         },
    //         Ordering::Equal => return mid,
    //     }
    // }
    // result

    // or newton's integer method
    let x = i64::from(x);
    let mut r = x as i64;

    while r * r > x {
        r = (r + x / r) / 2;
    }

    r as i32
}

impl Solution {
    #[must_use]
    pub fn my_sqrt(x: i32) -> i32 {
        my_sqrt(x)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0069::my_sqrt;

    #[test]
    fn test_1() {
        assert_eq!(my_sqrt(4), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(my_sqrt(8), 2);
    }
}
