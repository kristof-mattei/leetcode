fn nth_ugly_number(n: i32) -> i32 {
    let mut v: Vec<i32> = Vec::with_capacity(n as usize);
    v.push(1);

    let mut p = [0, 0, 0];

    while v.len() < n as usize {
        let first = v[p[0]] * 2;
        let second = v[p[1]] * 3;
        let third = v[p[2]] * 5;

        let nums = [first, second, third];

        let &min = nums.iter().min().unwrap();

        v.push(min);

        for i in 0..nums.len() {
            if nums[i] == min {
                p[i] += 1;
            }
        }
    }

    v.pop().unwrap()
}

impl Solution {
    #[must_use]
    pub fn nth_ugly_number(n: i32) -> i32 {
        nth_ugly_number(n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::nth_ugly_number;

    #[test]
    fn test_1() {
        assert_eq!(nth_ugly_number(10), 12);
    }

    #[test]
    fn test_2() {
        assert_eq!(nth_ugly_number(1), 1);
    }
}
