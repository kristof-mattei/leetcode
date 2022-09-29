use std::cmp::Ordering;

fn compare_version(version1: &str, version2: &str) -> i32 {
    let mut split_v1 = version1.split('.');
    let mut split_v2 = version2.split('.');

    loop {
        match (
            split_v1.next().map(|v1| v1.parse::<u32>().unwrap()),
            split_v2.next().map(|v2| v2.parse::<u32>().unwrap()),
        ) {
            | (None, None) => return 0,
            | (None, Some(0)) | (Some(0), None) => {},
            | (None, Some(_)) => return -1,
            | (Some(_), None) => return 1,
            | (Some(v1_piece), Some(v2_piece)) => match v1_piece.cmp(&v2_piece) {
                | Ordering::Less => return -1,
                | Ordering::Equal => {},
                | Ordering::Greater => return 1,
            },
        };
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn compare_version(version1: String, version2: String) -> i32 {
        compare_version(&version1, &version2)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0165::compare_version;

    #[test]
    fn test_1() {
        assert_eq!(compare_version("1.01", "1.001"), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(compare_version("1.0", "1.0.0"), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(compare_version("0.1", "1.1"), -1);
    }

    #[test]
    fn test_4() {
        assert_eq!(compare_version("1.0.1", "1"), 1);
    }
}
