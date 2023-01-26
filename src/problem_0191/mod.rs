fn debug(n: u32, mask: u32, shift: u32) -> u32 {
    // println!("---");
    // println!("n before: {n:032b}");
    // println!("shift   : {shift}");
    // println!("mask    : {mask:032b}");
    // println!("n mask  : {:032b}", n & mask);
    // println!("n mask s: {:032b}", (n >> shift) & mask);

    (n & mask) + ((n >> shift) & mask)
}
fn hamming_weight(mut n: u32) -> i32 {
    n = debug(n, 0x5555_5555, 1); // put count of each  2 bits into those  2 bits
    n = debug(n, 0x3333_3333, 2); // put count of each  4 bits into those  4 bits
    n = debug(n, 0x0F0F_0F0F, 4); // put count of each  8 bits into those  8 bits
    n = debug(n, 0x00FF_00FF, 8); // put count of each 16 bits into those 16 bits
    n = debug(n, 0x0000_FFFF, 16); // put count of each 32 bits into those 32 bits
    n as i32
}
impl Solution {
    #[must_use]
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        hamming_weight(n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::hamming_weight;

    #[test]
    fn test_1() {
        assert_eq!(hamming_weight(0b0000_0000_0000_0000_0000_0000_0000_1011), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(hamming_weight(0b0000_0000_0000_0000_0000_0000_1000_0000), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            hamming_weight(0b1111_1111_1111_1111_1111_1111_1111_1101),
            31
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(hamming_weight(0b0100_0000_0000_0000_0000_0000_1000_0000), 2);
    }
}
