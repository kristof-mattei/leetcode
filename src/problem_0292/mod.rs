impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn can_win_nim(n: i32) -> bool {
        can_win_nim(n)
    }
}

fn can_win_nim(n: i32) -> bool {
    n % 4 == 0
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0292::can_win_nim;

    #[test]
    fn test_1() {
        assert!(can_win_nim(4));
    }
}
