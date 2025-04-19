impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value)]
    pub fn is_additive_number(input: String) -> bool {
        is_additive_number(&input)
    }
}

fn is_additive_number_r(a: u64, b: u64, rest: &[u8]) -> bool {
    if rest.is_empty() {
        return true;
    }

    let c = to_digits(a + b);

    // strip_prefix doesn't work for &[T]... why not?
    rest.starts_with(&c) && is_additive_number_r(b, a + b, &rest[c.len()..])
}

fn to_digits(mut i: u64) -> Vec<u8> {
    let mut chars = vec![];

    while i > 0 {
        let diff = i % 10;

        chars.push(u8::try_from(diff).unwrap());

        i /= 10;
    }

    chars.reverse();

    chars
}

fn to_int(digits: &[u8]) -> u64 {
    let mut sum = 0;

    for d in digits {
        sum *= 10;
        sum += u64::from(*d);
    }

    sum
}

fn is_additive_number(input: &str) -> bool {
    let num = input.as_bytes().iter().map(|b| b - 48).collect::<Vec<_>>();

    for split in 1..num.len() {
        for end in split + 1..num.len() {
            if num[0] == 0 && split > 1 {
                // for the left number we only consider a leading 0 as just 0 (i.e. split = 1), but 01 is always invalid
                return false;
            }

            if num[split] == 0 && end > split + 1 {
                // for the right number, same, 0 is ok, but leading 0s are not
                // here we don't stop, as we can move the split right
                continue;
            }

            let left = to_int(&num[0..split]);
            let right = to_int(&num[split..end]);

            if is_additive_number_r(left, right, &num[end..]) {
                return true;
            }
        }
    }

    false
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0306::is_additive_number;

    #[test]
    fn test_1() {
        assert!(is_additive_number("112358"));
    }

    #[test]
    fn test_2() {
        assert!(is_additive_number("199100199"));
    }

    #[test]
    fn test_3() {
        assert!(is_additive_number("11111111111011111111111"));
    }
}
