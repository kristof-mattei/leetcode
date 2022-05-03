fn diff_ways_to_compute(expression: &str) -> Vec<i32> {
    if expression.len() <= 2 {
        return vec![expression.parse::<i32>().unwrap()];
    }

    let mut result: Vec<i32> = Vec::new();

    for i in 0..expression.len() {
        if !expression.as_bytes()[i].is_ascii_digit() {
            let left = diff_ways_to_compute(&expression[..i]);
            let right = diff_ways_to_compute(&expression[i + 1..]);

            for a in &left {
                for b in &right {
                    result.push(match expression.as_bytes()[i] {
                        b'+' => a + b,
                        b'-' => a - b,
                        b'*' => a * b,
                        _ => unreachable!(),
                    });
                }
            }
        }
    }

    result
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        diff_ways_to_compute(&expression)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    use super::diff_ways_to_compute;

    #[test]
    fn test_1() {
        assert_eq!(diff_ways_to_compute("2-1-1"), [0, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(diff_ways_to_compute("2*3-4*5"), [-34, -14, -10, -10, 10]);
    }
}
