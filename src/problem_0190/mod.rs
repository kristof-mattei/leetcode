fn reverse_bits(x: u32) -> u32 {
    let mut new: u32 = 0;

    for i in 0..32 {
        let bit = (x >> i) & 1;
        if bit == 1 {
            let shifted = bit << (31 - i);
            new |= shifted;
        }
    }

    new
}

impl Solution {
    #[must_use]
    pub fn reverse_bits(x: u32) -> u32 {
        reverse_bits(x)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    use crate::problem_0190::reverse_bits;

    #[test]
    fn test_1() {
        assert_eq!(reverse_bits(43_261_596), 964_176_192);
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse_bits(4_294_967_293), 3_221_225_471);
    }

    #[test]
    fn test_3() {
        assert_eq!(reverse_bits(1), 2_147_483_648);
    }
}
