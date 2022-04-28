fn get_row(num_rows: usize) -> Vec<i32> {
    let mut result = vec![vec![1]];
    for i in 1..=num_rows {
        let mut temp = vec![1];

        for w in result[i - 1].windows(2) {
            temp.push(w[0] + w[1]);
        }

        temp.push(1);

        result.push(temp);
    }

    result.remove(num_rows)
}

impl Solution {
    #[must_use]
    pub fn get_row(num_rows: i32) -> Vec<i32> {
        get_row(num_rows as usize)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0119::get_row;

    #[test]
    fn test_1() {
        assert_eq!(get_row(3), vec![1, 3, 3, 1],);
    }

    #[test]
    fn test_2() {
        assert_eq!(get_row(0), vec![1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(get_row(1), vec![1, 1]);
    }
}
