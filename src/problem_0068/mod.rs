fn full_justify_one_line(words: &[String], max_width: usize) -> String {
    if words.len() == 1 {
        return left_justify(words, max_width);
    }

    let gaps_to_fill = words.len() - 1;

    let spaces_available = max_width - words.iter().map(String::len).sum::<usize>();

    let spaces_per_word = spaces_available / gaps_to_fill;

    let mut excess_space = spaces_available - (spaces_per_word * gaps_to_fill);

    let mut s = String::new();

    for (i, w) in words.iter().enumerate() {
        s.push_str(w);

        // don't add spaces for the last word
        if i != gaps_to_fill {
            std::iter::repeat(' ')
                .take(spaces_per_word)
                .for_each(|c| s.push(c));

            if excess_space > 0 {
                excess_space -= 1;
                s.push(' ');
            }
        }
    }

    s
}

fn left_justify(words: &[String], max_width: usize) -> String {
    let mut s = words.join(" ");

    std::iter::repeat(' ')
        .take(max_width - s.len())
        .for_each(|c| s.push(c));

    s
}

fn full_justify(words: Vec<String>, max_width: usize) -> Vec<String> {
    let mut r = Vec::new();

    let mut c = Vec::new();

    for w in words {
        if c.len() + c.iter().map(String::len).sum::<usize>() + w.len() > max_width {
            // she's full, move spaces
            r.push(full_justify_one_line(&c, max_width));
            c.clear();
        }
        c.push(w);
    }

    r.push(left_justify(&c, max_width));
    r
}

impl Solution {
    #[must_use]
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        full_justify(words, max_width as usize)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0068::full_justify;

    #[test]
    fn test_1() {
        assert_eq!(
            full_justify(
                [
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
                16
            ),
            &["This    is    an", "example  of text", "justification.  "]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            full_justify(
                ["What", "must", "be", "acknowledgment", "shall", "be"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
                16
            ),
            &["What   must   be", "acknowledgment  ", "shall be        "]
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            full_justify(
                [
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
                20
            ),
            &[
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
        );
    }
}
