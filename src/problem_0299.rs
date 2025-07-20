impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn get_hint(secret: String, guess: String) -> String {
        get_hint(&secret, &guess)
    }
}

fn get_hint(secret: &str, guess: &str) -> String {
    let mut bulls = 0;
    let mut cows = 0;

    let secret = secret
        .chars()
        .map(|v| usize::try_from(v.to_digit(10).unwrap()).unwrap())
        .collect::<Vec<_>>();
    let guess = guess
        .chars()
        .map(|v| usize::try_from(v.to_digit(10).unwrap()).unwrap())
        .collect::<Vec<_>>();

    let mut numbers = [0_i32; 10];

    for (&s, &g) in secret.iter().zip(guess.iter()) {
        if s == g {
            bulls += 1;
        } else {
            if numbers[s] < 0 {
                cows += 1;
            }

            if numbers[g] > 0 {
                cows += 1;
            }

            numbers[s] += 1;
            numbers[g] -= 1;
        }
    }

    format!("{}A{}B", bulls, cows)
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0299::get_hint;

    #[test]
    fn test_1() {
        assert_eq!(get_hint("1807", "7810"), "1A3B");
    }

    #[test]
    fn test_2() {
        assert_eq!(get_hint("1123", "0111"), "1A1B");
    }
}
