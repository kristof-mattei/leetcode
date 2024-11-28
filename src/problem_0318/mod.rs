impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_product(words: Vec<String>) -> i32 {
        max_product(&words)
    }
}

fn max_product(words: &[String]) -> i32 {
    let mut words_mapping = Vec::with_capacity(words.len());

    for word in words {
        let mut mapping = 0;

        // only works in ASCII
        for byte in word.as_bytes() {
            mapping |= 1 << (byte - b'a');
        }

        words_mapping.push(mapping);
    }

    let mut max = 0;

    for left in 0..words.len() {
        for right in (left + 1)..words.len() {
            if words_mapping[left] & words_mapping[right] != 0 {
                continue;
            }

            max = max.max(words[left].len() * words[right].len());
        }
    }

    max as i32
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0318::max_product;

    #[test]
    fn test_1() {
        assert_eq!(
            max_product(
                &["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
            ),
            16
        );
    }
}
