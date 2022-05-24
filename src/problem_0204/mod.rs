fn count_primes(n: i32) -> i32 {
    if n < 2 {
        return 0;
    }

    let mut is_prime = vec![true; n as usize];
    let mut count = n - 2;
    is_prime[0] = false;
    is_prime[1] = false;

    #[allow(clippy::range_plus_one)]
    for i in 2..f64::sqrt(f64::from(n)).ceil() as usize {
        if is_prime[i] {
            for j in ((i * i)..n as usize).step_by(i) {
                is_prime[j] = false;
                count -= 1;
            }
        }
    }

    count as i32
}

impl Solution {
    #[must_use]
    pub fn count_primes(n: i32) -> i32 {
        count_primes(n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0204::count_primes;

    #[test]
    fn test_1() {
        assert_eq!(count_primes(10), 4);
    }
}
