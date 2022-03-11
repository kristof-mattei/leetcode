use crate::shared::Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn is_match(s: String, p: String) -> bool {
        is_match(&s, &p)
    }
}
enum SingleToken {
    Wildcard,
    Char(char),
}

enum Token {
    SingleToken(SingleToken),
    ZeroOrMore(SingleToken),
}

fn to_simple_token(c: char) -> SingleToken {
    match c {
        '.' => SingleToken::Wildcard,
        x => SingleToken::Char(x),
    }
}

fn tokenize(p: &str) -> Vec<Token> {
    let mut chars = p.chars().into_iter().collect::<Vec<_>>();
    let mut tokenized = Vec::new();
    while let Some(c) = chars.pop() {
        let token = match c {
            '*' => {
                let next_token = to_simple_token(chars.pop().unwrap());
                Token::ZeroOrMore(next_token)
            },
            x => Token::SingleToken(to_simple_token(x)),
        };
        tokenized.push(token);
    }

    tokenized.reverse();
    tokenized
}

fn is_match(s: &str, p: &str) -> bool {
    let tokenized_regex = tokenize(p);
    is_match_r(&s.chars().collect::<Vec<_>>(), &tokenized_regex)
}

fn is_match_r(remainder: &[char], remaining_regex: &[Token]) -> bool {
    let mut index: usize = 0;
    let mut regex_index: usize = 0;
    while let Some(token) = remaining_regex.get(regex_index) {
        match token {
            Token::SingleToken(t) => {
                match remainder.get(index) {
                    Some(c) => {
                        if let SingleToken::Char(expected_char) = t {
                            if expected_char != c {
                                return false;
                            }
                        }
                    },
                    None => return false,
                }
                index += 1;
            },
            Token::ZeroOrMore(st) => {
                match st {
                    SingleToken::Char(expected_char) => {
                        // we walk for as long as we find
                        // our current character, and once at the end
                        // we check if our next token requests that character too. If so, move one index back so that one can be satisfied
                        let mut highest_continuation = index;

                        while let Some(c) = remainder.get(highest_continuation) {
                            if expected_char == c {
                                highest_continuation += 1;
                            } else {
                                break;
                            }
                        }

                        // optimization
                        if index != highest_continuation {
                            // now we need to resume the remainer of the regex for each of the contunation points
                            // starting from the last one to ensure we're as greedy as possible

                            // range is inclusive as we start from the index (i.e. no match for the current <char>* all the way until AFTER the last matching char)
                            return (index..=highest_continuation).rev().any(|c| {
                                is_match_r(&remainder[c..], &remaining_regex[regex_index + 1..])
                            });
                        }
                    },
                    SingleToken::Wildcard => {
                        // optimization
                        if remaining_regex.get(regex_index + 1).is_none() {
                            return true;
                        }
                        // this one is tricky.
                        // we walk until we have a character that matches the next token
                        // but that is not enough. For example:
                        // .*c with the string zzzzzzzczzzzc
                        // if we stop at the first, then the string wouldn't be considered valid.
                        // we also can't just go to the last. For example:
                        // .*cabc with the string zzzzzzzczzzzcabc
                        // because then we would just walk to the last c, leaving no string anymore for abc...
                        // so what we'll need to do is for each occurence of 'next_token' we need to check if we can match
                        // the remainer of the regex against the remainder of the string, starting from the end
                        // as we want to be as greedy as possible

                        // now we need to resume the remainer of the regex for each of the contunation points
                        // starting from the last one to ensure we're as greedy as possible

                        // inclusive range because we also want to test an emtpy string
                        // in the case the remaining tokens are all of the form of <char>* or .*
                        return (index..=remainder.len()).rev().any(|c| {
                            is_match_r(&remainder[c..], &remaining_regex[regex_index + 1..])
                        });
                    },
                }
            },
        }
        regex_index += 1;
    }
    index == remainder.len() && regex_index == remaining_regex.len()
}

#[cfg(test)]
mod tests {
    use crate::problem_0010::is_match;

    #[test]
    fn first() {
        assert!(!is_match("aa", "a"));
    }
    #[test]
    fn second() {
        assert!(is_match("aa", "a*"));
    }
    #[test]
    fn third() {
        assert!(is_match("ab", ".*"));
    }
    #[test]
    fn fourth() {
        assert!(!is_match("ab", "abc"));
    }
    #[test]
    fn fifth() {
        assert!(is_match("abc", ".*c"));
    }
    #[test]
    fn six() {
        assert!(is_match("abcccccccd", ".*c*d"));
    }
    #[test]
    fn seven() {
        assert!(is_match("aaa", "a*a"));
    }
    #[test]
    fn eight() {
        assert!(is_match("aab", "a*b"));
    }
    #[test]
    fn nine() {
        assert!(is_match("mississippi", "mis*is*ip*."));
    }
    #[test]
    fn ten() {
        assert!(is_match("aaa", "ab*a*c*a"));
    }
    #[test]
    fn eleven() {
        assert!(is_match("abcaaaaaaabaabcabac", ".*ab.a.*a*a*.*b*b*"));
    }
    #[test]
    fn eleven_partial() {
        assert!(is_match("abcabac", "ab.a.*a*a*.*b*b*"));
    }
    #[test]
    fn eleven_partial_2() {
        assert!(is_match("bac", ".*a*a*.*b*b*"));
    }
    #[test]
    fn eleven_partial_3() {
        assert!(is_match("bac", ".*b*b*"));
    }
}
