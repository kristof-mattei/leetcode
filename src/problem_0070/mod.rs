use std::collections::HashMap;

fn climb_stairs_r(cache: &mut HashMap<i32, i32>, n: i32) -> i32 {
    if cache.contains_key(&n) {
        return cache[&n];
    }

    let result = match n {
        | 0 => 0,
        | 1 => 1,
        | 2 => 2,
        | _ => climb_stairs_r(cache, n - 1) + climb_stairs_r(cache, n - 2),
    };

    cache.insert(n, result);
    result
}

fn climb_stairs(n: i32) -> i32 {
    let mut cache = HashMap::<i32, i32>::new();
    climb_stairs_r(&mut cache, n)
}

impl Solution {
    #[must_use]
    pub fn climb_stairs(n: i32) -> i32 {
        climb_stairs(n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0070::climb_stairs;

    #[test]
    fn test_1() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(climb_stairs(3), 3);
    }
}
