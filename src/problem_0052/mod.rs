use crate::problem_0051::solve_n_queens;

fn total_n_queens(n: i32) -> i32 {
    solve_n_queens(n).len() as i32
}

impl Solution {
    #[must_use]
    pub fn total_n_queens(n: i32) -> i32 {
        total_n_queens(n)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0052::total_n_queens;

    #[test]
    fn test_1() {
        assert_eq!(total_n_queens(4), 2);
    }
}
