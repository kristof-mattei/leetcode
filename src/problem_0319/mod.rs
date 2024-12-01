impl Solution {
    #[must_use]
    pub fn bulb_switch(n: i32) -> i32 {
        bulb_switch(n)
    }
}

fn bulb_switch(n: i32) -> i32 {
    f64::from(n).sqrt() as i32
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0319::bulb_switch;

    #[test]
    fn test_1() {
        assert_eq!(bulb_switch(3), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(bulb_switch(0), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(bulb_switch(1), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(bulb_switch(99999), 316);
    }
}
