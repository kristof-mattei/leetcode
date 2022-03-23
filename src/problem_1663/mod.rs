use crate::shared::Solution;

fn get_smallest_string(n: i32, k: i32) -> String {
    let mut z = 0;

    // maximize the 'z's so that we just have enough left
    // to fill the remaining slots
    while (k - (z + 1) * 26) > n - 1 - z {
        z += 1;
    }

    // we start with as many 'a's as we want, minus the 'z's and minus 1 for the middle
    let a = n - z - 1;

    let mut result = vec![b'a'; a as usize];

    // 'k' minus (the amount of 'z's times 26) minus all cells that are NOT 'z'
    let middle = (k - (z * 26) - a) as u8;

    // we do - 1 since in middle 1-based ('a' = 1, 'b' = 2, ...)
    // but to print it we need 0-based
    result.push(middle - 1 + b'a');

    result.resize(result.len() + z as usize, b'z');

    unsafe { String::from_utf8_unchecked(result) }
}

impl Solution {
    #[must_use]
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        get_smallest_string(n, k)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_1663::get_smallest_string;

    #[test]
    fn test_1() {
        assert_eq!(get_smallest_string(3, 27), "aay");
    }

    #[test]
    fn test_2() {
        assert_eq!(get_smallest_string(5, 73), "aaszz");
    }

    #[test]
    fn test_3() {
        assert_eq!(get_smallest_string(5, 31), "aaabz");
    }

    #[test]
    fn test_4() {
        assert_eq!(get_smallest_string(3, 29), "abz");
    }
}
