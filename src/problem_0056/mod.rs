#[derive(Clone, Copy)]
enum R {
    Empty,
    Start,
    Occupied,
    End,
    Single,
}

// alt
fn merge_alt(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut arr = [R::Empty; 10001];

    for i in intervals {
        let int_start = i[0] as usize;
        let int_end = i[1] as usize;

        if int_start == int_end {
            // single, ugh
            let single = &mut arr[int_start];
            if let R::Empty = single {
                *single = R::Single;
            };

            continue;
        }

        let start = &mut arr[int_start];
        match start {
            | R::Empty | R::Single => *start = R::Start,
            | R::End => *start = R::Occupied,
            | _ => {},
        };

        for item in arr.iter_mut().take(int_end).skip(int_start + 1) {
            *item = R::Occupied;
        }

        let end = &mut arr[int_end];
        match end {
            | R::Empty | R::Single => *end = R::End,
            | R::Start => *end = R::Occupied,
            | _ => {},
        }
    }

    let mut min = 0;
    let mut result = Vec::new();
    for (i, r) in arr.iter().enumerate() {
        match r {
            | R::Start => {
                min = i;
            },
            | R::End => result.push(vec![min as i32, i as i32]),
            | R::Single => result.push(vec![i as i32, i as i32]),
            | _ => {},
        }
    }

    result
}

fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_unstable();

    let mut result = vec![];
    let mut last = intervals[0].clone();

    for i in intervals.into_iter().skip(1) {
        if last[1] >= i[0] {
            last[1] = last[1].max(i[1]);
        } else {
            result.push(last);
            last = i;
        }
    }

    result.push(last);

    result
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        merge(intervals)
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn merge_alt(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        merge_alt(intervals)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0056::{merge, merge_alt};

    #[test]
    fn test_1() {
        let result = merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]);

        assert_eq!(result, [[1, 6], [8, 10], [15, 18]]);
    }

    #[test]
    fn test_1_alt() {
        let result = merge_alt(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]);

        assert_eq!(result, [[1, 6], [8, 10], [15, 18]]);
    }

    #[test]
    fn test_2() {
        let result = merge(vec![
            vec![2, 3],
            vec![4, 5],
            vec![6, 7],
            vec![8, 9],
            vec![1, 10],
        ]);

        assert_eq!(result, [[1, 10]]);
    }

    #[test]
    fn test_2_alt() {
        let result = merge_alt(vec![
            vec![2, 3],
            vec![4, 5],
            vec![6, 7],
            vec![8, 9],
            vec![1, 10],
        ]);

        assert_eq!(result, [[1, 10]]);
    }

    #[test]
    fn test_3() {
        let result = merge(vec![vec![1, 4], vec![0, 0]]);

        assert_eq!(result, [[0, 0], [1, 4]]);
    }

    #[test]
    fn test_3_alt() {
        let result = merge_alt(vec![vec![1, 4], vec![0, 0]]);

        assert_eq!(result, [[0, 0], [1, 4]]);
    }

    #[test]
    fn test_4() {
        let result = merge(vec![vec![1, 4], vec![4, 5]]);

        assert_eq!(result, [[1, 5]]);
    }

    #[test]
    fn test_4_alt() {
        let result = merge_alt(vec![vec![1, 4], vec![4, 5]]);

        assert_eq!(result, [[1, 5]]);
    }

    #[test]
    fn test_5() {
        let result = merge(vec![vec![1, 3], vec![1, 7], vec![4, 5]]);

        assert_eq!(result, [[1, 7]]);
    }

    #[test]
    fn test_5_alt() {
        let result = merge_alt(vec![vec![1, 3], vec![1, 7], vec![4, 5]]);

        assert_eq!(result, [[1, 7]]);
    }
}
