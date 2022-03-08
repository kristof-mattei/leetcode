use crate::shared::Solution;

use std::cmp::Ordering;

fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rest = intervals.split_off(1);

    let mut result = intervals;

    for i in rest {
        let last = result.last_mut().unwrap();

        if last[1] >= i[0] {
            last[1] = last[1].max(i[1]);
        } else {
            result.push(i);
        }
    }

    result
}

fn previous_one_upper_bound_overlaps_current(
    list: &[Vec<i32>],
    index: usize,
    lower_bound: i32,
) -> bool {
    index > 0 && list[index - 1][1] >= lower_bound
}

fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let first_larger_interval = intervals
        .iter()
        .enumerate()
        .find_map(|(i, interval)| {
            match interval[0].cmp(&new_interval[0]) {
                // for calculation the one before doesn't matter
                // as they are non-overlapping, due to the question's guarantees
                Ordering::Equal => Some((i, false)),
                Ordering::Greater => {
                    // if the interval is greater than our new one
                    // we want to check include the one before
                    // becaue the one before might overlap with ours, and we're doing everything
                    // items to be recalculated
                    // e.g.: [[1,5], [6,10]], and new_interval is [4,7].
                    // we would find [6,10] as the first one where [0] (i.e. 6) is larger than
                    // 4, however we want to include the one before if its [1] is larger than ours
                    // we use saturating_sub to prevent having 0 - 1, which would overflow
                    let p =
                        previous_one_upper_bound_overlaps_current(&intervals, i, new_interval[0]);

                    Some((if p { i - 1 } else { i }, p))
                },
                Ordering::Less => None,
            }
        })
        .unwrap_or_else(|| {
            let i = intervals.len();

            let p = previous_one_upper_bound_overlaps_current(&intervals, i, new_interval[0]);

            (if p { i - 1 } else { i }, p)
        });

    // we then split it. `intervals` will have [0..first_larger_interval]
    // `found_and_rest` will have [first_larger_interval..];
    let found_and_rest = intervals.split_off(first_larger_interval.0);

    let mut iter = found_and_rest.into_iter();

    // if .1 is true we do <item>, new_interval, <rest>
    // otherwise <new_interval>, <rest>
    let mut temp = if first_larger_interval.1 {
        vec![iter.by_ref().next().unwrap()]
    } else {
        vec![]
    };

    temp.push(new_interval);

    // add back the rest
    temp.append(&mut iter.collect::<Vec<_>>());

    // re-sort
    temp = merge(temp);

    // add back to the original intervals
    intervals.append(&mut temp);

    intervals
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        insert(intervals, new_interval)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_57::insert;

    #[test]
    fn test_1() {
        let result = insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]);

        assert_eq!(result, [[1, 5], [6, 9]]);
    }

    #[test]
    fn test_2() {
        let result = insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            vec![4, 8],
        );

        assert_eq!(result, [[1, 2], [3, 10], [12, 16]]);
    }
}
