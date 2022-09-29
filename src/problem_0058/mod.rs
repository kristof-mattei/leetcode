fn length_of_last_word(s: &str) -> i32 {
    let trimmed = s.trim_end();

    let l = match trimmed.rsplit_once(' ') {
        Some((_, r)) => r.len(),
        None => trimmed.len(),
    };

    l as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn length_of_last_word(s: String) -> i32 {
        length_of_last_word(&s)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0058::length_of_last_word;

    #[test]
    fn test_1() {
        assert_eq!(length_of_last_word("Hello World"), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(length_of_last_word("   fly me   to   the moon  "), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(length_of_last_word("luffy is still joyboy"), 6);
    }

    #[test]
    fn test_4() {
        assert_eq!(length_of_last_word("monitor"), 7);
    }

    #[test]
    fn test_5() {
        assert_eq!(length_of_last_word("monitor           "), 7);
    }
    #[test]
    fn test_6() {
        assert_eq!(length_of_last_word("              monitor           "), 7);
    }
}
