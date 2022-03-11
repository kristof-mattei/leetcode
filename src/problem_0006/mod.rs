use crate::shared::Solution;

fn convert(s: &str, num_rows: usize) -> String {
    if num_rows == 1 {
        return s.to_string();
    }

    let mut rows: Vec<Vec<char>> = vec![vec![]; num_rows];

    let mut current_row = 0;
    let mut going_up = false;

    for c in s.chars() {
        rows[current_row].push(c);
        if going_up {
            if current_row == 0 {
                going_up = false;
                current_row = 1;
            } else {
                current_row -= 1;
            }
        } else if current_row + 1 == (num_rows) {
            going_up = true;
            current_row -= 1;
        } else {
            current_row += 1;
        }
    }

    rows.iter().flatten().collect::<String>()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn convert(s: String, num_rows: i32) -> String {
        convert(&s, num_rows as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0006::convert;

    #[test]
    fn test_1() {
        assert_eq!(convert("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR");
    }

    #[test]
    fn test_2() {
        assert_eq!(convert("PAYPALISHIRING", 4), "PINALSIGYAHRPI");
    }

    #[test]
    fn test_3() {
        assert_eq!(convert("A", 1), "A");
    }
}
