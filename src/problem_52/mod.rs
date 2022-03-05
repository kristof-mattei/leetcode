use crate::problem_51::solve_n_queens;
use crate::shared::Solution;

fn total_n_queens(n: i32) -> i32 {
    solve_n_queens(n).len() as i32
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        total_n_queens(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_52::total_n_queens;

    #[test]
    fn test_1() {
        assert_eq!(total_n_queens(4), 2);
    }
}
