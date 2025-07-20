fn is_happy(mut n: i32) -> bool {
    let mut seen = vec![];

    loop {
        let mut new = 0;

        while n > 0 {
            new += i32::pow(n % 10, 2);
            n /= 10;
        }

        if new == 1 {
            return true;
        }

        if seen.contains(&new) {
            return false;
        }

        seen.push(new);

        // reset
        n = new;
    }
}

impl Solution {
    #[must_use]
    pub fn is_happy(n: i32) -> bool {
        is_happy(n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0202::is_happy;

    #[test]
    fn test_1() {
        assert!(is_happy(19));
    }

    #[test]
    fn test_2() {
        assert!(!is_happy(2));
    }

    #[test]
    fn test_3() {
        for i in 0..200 {
            println!("{i}: {}", is_happy(i));
        }
    }
}
