fn generate(num_rows: usize) -> Vec<Vec<i32>> {
    let mut result = vec![vec![1]];
    for i in 1..num_rows {
        let mut temp = vec![1];

        for w in result[i - 1].windows(2) {
            temp.push(w[0] + w[1]);
        }

        temp.push(1);

        result.push(temp);
    }
    result
}

impl Solution {
    #[must_use]
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        generate(num_rows as usize)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0118::generate;

    #[test]
    fn test_1() {
        assert_eq!(
            generate(5),
            [
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(generate(1), [vec![1],]);
    }
}
