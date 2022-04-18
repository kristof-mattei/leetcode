use crate::shared::Solution;

fn reverse_words<T: AsRef<str>>(s: T) -> String {
    let bytes = s.as_ref().as_bytes();

    let mut reversed = Vec::new();

    let mut index = bytes.len();
    let mut in_word = false;

    let mut last_char_index = bytes.len() + 1;

    loop {
        if index == 0 || bytes[index - 1] == b' ' {
            if in_word {
                reversed.push(String::from_utf8_lossy(&bytes[index..last_char_index - 1]));
                in_word = false;
            }
            last_char_index = index;
        } else {
            // move on
            in_word = true;
        }

        if index == 0 {
            break;
        }
        index -= 1;
    }

    reversed.join(" ")
}

impl Solution {
    #[must_use]
    pub fn reverse_words(s: String) -> String {
        reverse_words(s)
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_words;

    #[test]
    fn test_1() {
        let input = "the sky is blue";
        let expected = "blue is sky the";

        assert_eq!(reverse_words(input), expected);
    }

    #[test]
    fn test_2() {
        let input = "  hello world  ";
        let expected = "world hello";

        assert_eq!(reverse_words(input), expected);
    }
    #[test]
    fn test_3() {
        let input = "a good   example";
        let expected = "example good a";

        assert_eq!(reverse_words(input), expected);
    }
}
