use crate::shared::Solution;

fn count_and_say_r(n: i32) -> Vec<u32> {
    if n == 1 {
        return vec![1];
    }

    let c_a_s = count_and_say_r(n - 1);
    let mut n = Vec::new();

    for o in c_a_s {
        match n.last_mut() {
            Some((number, count)) if *number == o => {
                *count += 1;
            },
            Some(_) | None => n.push((o, 1u32)),
        }
    }

    n.iter().flat_map(|&(n, c)| [c, n]).collect()
}

fn count_and_say(n: i32) -> String {
    count_and_say_r(n)
        .iter()
        .map(|&v| ((v as u8 + b'0') as char))
        .collect::<String>()
}

impl Solution {
    #[must_use]
    pub fn count_and_say(n: i32) -> String {
        count_and_say(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0038::count_and_say;

    #[test]
    fn test_1() {
        assert_eq!(count_and_say(1), "1".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(count_and_say(4), "1211".to_string());
    }
}
