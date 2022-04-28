fn add(long: &str, short: &str) -> String {
    let mut carry = false;

    let mut long_v = long.bytes().collect::<Vec<_>>();
    let short_v = short.as_bytes();
    let diff = long_v.len() - short_v.len();

    for i in (0..long_v.len()).rev() {
        let l = long_v[i] == b'1';

        let s = {
            if i < diff {
                if !carry {
                    return String::from_utf8(long_v).unwrap();
                }

                false
            } else {
                short_v[i - diff] == b'1'
            }
        };

        // (long_v[i], carry) = match &[l, s, carry].iter().filter(|&&v| v).count() {

        let (b, c) = match &[l, s, carry].iter().filter(|&&v| v).count() {
            0 => (b'0', false),
            1 => (b'1', false),
            2 => (b'0', true),
            3 => (b'1', true),
            _ => unreachable!(),
        };

        long_v[i] = b;
        carry = c;
    }

    if carry {
        long_v.insert(0, b'1');
    }
    String::from_utf8(long_v).unwrap()
}

fn add_binary(a: &str, b: &str) -> String {
    if a.len() > b.len() {
        add(a, b)
    } else {
        add(b, a)
    }
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn add_binary(a: String, b: String) -> String {
        add_binary(&a, &b)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0067::add_binary;

    #[test]
    fn test_1() {
        assert_eq!(add_binary("11", "1"), "100");
    }
    #[test]
    fn test_2() {
        assert_eq!(add_binary("1010", "1011"), "10101");
    }
}
