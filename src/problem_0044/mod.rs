use std::collections::HashMap;

fn is_match(s: &str, p: &str) -> bool {
    is_match_r(
        &mut HashMap::new(),
        &s.chars().collect::<Vec<_>>(),
        &p.chars().collect::<Vec<_>>(),
    )
}

fn memoize(
    cache: &mut HashMap<(usize, usize), bool>,
    remainder: &[char],
    remaining_regex: &[char],
) -> bool {
    let key = (remainder.len(), remaining_regex.len());

    let result = cache.get(&key);

    if let Some(&c) = result {
        return c;
    }

    let is_match = is_match_r(cache, remainder, remaining_regex);

    cache.insert(key, is_match);

    is_match
}

fn is_match_r(
    cache: &mut HashMap<(usize, usize), bool>,
    remainder: &[char],
    remaining_regex: &[char],
) -> bool {
    let token = match remaining_regex.first() {
        Some(t) => t,
        None => return remainder.is_empty() && remaining_regex.is_empty(),
    };

    match token {
        '?' if remainder.first().is_some() => {
            // here
            memoize(cache, &remainder[1..], &remaining_regex[1..])
        },
        '?' => false,
        c @ 'a'..='z' => {
            if let Some(e_c) = remainder.first() {
                if e_c == c {
                    return memoize(cache, &remainder[1..], &remaining_regex[1..]);
                }
            }
            false
        },

        '*' => {
            let mut offset = 0;

            for rr in remaining_regex.iter().skip(1) {
                if rr != &'*' {
                    break;
                }
                offset += 1;
            }

            // optimization
            if remaining_regex.get(offset + 1).is_none() {
                return true;
            }

            for c in (0..=remainder.len()).rev() {
                let is_match = memoize(cache, &remainder[c..], &remaining_regex[offset + 1..]);

                if is_match {
                    return true;
                }
            }

            false
        },
        _ => unreachable!(),
    }
}

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn is_match(s: String, p: String) -> bool {
        is_match(&s, &p)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0044::is_match;

    #[test]
    fn first() {
        assert!(!is_match("aa", "a"));
    }
    #[test]
    fn second() {
        assert!(is_match("aa", "*"));
    }

    #[test]
    fn third() {
        assert!(!is_match("cb", "?a"));
    }

    #[test]
    fn test_4() {
        assert!(is_match(
            "aaaabaaaabbbbaabbbaabbaababbabbaaaababaaabbbbbbaabbb\
             abababbaaabaabaaaaaabbaabbbbaababbababaabbbaababbbba",
            "*****b*aba***babaa*bbaba***a*aaba*b*aa**a*b**ba***a*a*"
        ));
    }

    #[test]
    fn test_5() {
        assert!(!is_match(
            "abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbaba\
             babaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababba\
             abbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaabab\
             aabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb",
            "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*\
             b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb"
        ));
    }

    #[test]
    fn test_6() {
        assert!(!is_match(
            "baababbaaaaabbababbbbbabaabaabaaabbaabbbbbbaabbbaaa\
             bbabbaabaaaaabaabbbaabbabababaaababbaaabaababbabaab\
             abbaababaabbbaaaaabbabbabababbbbaaaaaabaabbbbaababb\
             baabbaabbbbbbbbabbbabababbabababaaababbaaababaabb",
            "*ba***b***a*ab**b***bb*b***ab**aa***baba*b***bb**a*a\
             bbb*aa*b**baba**aa**b*b*a****aabbbabba*b*abaaa*aa**b"
        ));
    }

    #[test]
    fn test_7() {
        assert!(!is_match("aab", "c*a*b"));
    }
}
