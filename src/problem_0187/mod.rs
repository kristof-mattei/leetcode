use std::collections::HashMap;

fn find_repeated_dna_sequences(s: &str) -> Vec<String> {
    if s.len() <= 10 {
        return vec![];
    }

    let bytes = s.as_bytes();
    let mut counts = HashMap::new();

    for w in bytes.windows(10) {
        counts.entry(w).and_modify(|c| *c += 1).or_insert(1usize);
    }

    counts
        .into_iter()
        .filter(|(_, c)| *c >= 2)
        .map(|(w, _)| String::from_utf8(w.to_vec()).unwrap())
        .collect::<Vec<_>>()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        find_repeated_dna_sequences(&s)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::find_repeated_dna_sequences;

    #[test]
    fn test_1() {
        assert_eq!(
            HashSet::<String>::from_iter(find_repeated_dna_sequences(
                "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
            )),
            HashSet::from_iter(["AAAAACCCCC".to_string(), "CCCCCAAAAA".to_string()]),
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(find_repeated_dna_sequences("AAAAAAAAAAAAA"), ["AAAAAAAAAA"]);
    }

    #[test]
    fn test_3() {
        assert_eq!(find_repeated_dna_sequences("AAAAAAAAAA"), &[] as &[&str]);
    }

    #[test]
    fn test_4() {
        assert_eq!(find_repeated_dna_sequences("AAAAAAAAAAA"), &["AAAAAAAAAA"]);
    }
}
