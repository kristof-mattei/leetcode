use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn eaten_apples(apples: &[i32], days: &[i32]) -> i32 {
    let mut fresh_days_and_apples = BinaryHeap::<(Reverse<usize>, usize)>::new();

    let mut eaten = 0;

    for current_day in 0..days.len() {
        // pop the rotten ones
        while fresh_days_and_apples
            .peek()
            .map_or(false, |p| p.0 .0 <= current_day)
        {
            fresh_days_and_apples.pop();
        }

        // add apples to basket if we have apples
        if apples[current_day] > 0 {
            fresh_days_and_apples.push((
                Reverse(current_day + days[current_day] as usize),
                apples[current_day] as usize,
            ));
        }

        // eat one that expire the soonest
        if let Some(a) = fresh_days_and_apples.pop() {
            eaten += 1;

            // if that basket still has left, put it back (minus the one we ate)
            if a.1 > 1 {
                fresh_days_and_apples.push((Reverse(a.0 .0), (a.1 - 1)));
            }
        }
    }

    let mut day = days.len();

    // let's eat the remaining ones, one per day, but we can collapse
    // code because we don't need to add apples anymore
    while let Some(a) = fresh_days_and_apples.pop() {
        // only eat if they haven't expired
        if a.0 .0 > day {
            // calculate the apples we can eat, e.g.:
            // we're at day 10, the current apples are fresh for 12 days (i.e. 2 more days), and we have 3 apples in that basket.
            // we can only eat 2 of them, the last one will rot before we can eat it
            let can_eat_coming_days = a.1.min(a.0 .0 - day);

            eaten += can_eat_coming_days;

            // move to the next day that we don't have apples... yet (?) I guess we'll see next loop!
            day = a.0 .0.min(day + can_eat_coming_days);
        }
    }

    eaten as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        eaten_apples(&apples, &days)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_1705::eaten_apples;

    #[test]
    fn test_1() {
        assert_eq!(eaten_apples(&[1, 2, 3, 5, 2], &[3, 2, 1, 4, 2]), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(eaten_apples(&[3, 0, 0, 0, 0, 2], &[3, 0, 0, 0, 0, 2],), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(eaten_apples(&[2, 1, 10], &[2, 10, 1]), 4);
    }

    #[test]
    fn test_4() {
        assert_eq!(eaten_apples(&[3, 1, 1, 0, 0, 2], &[3, 1, 1, 0, 0, 2]), 5);
    }
}
