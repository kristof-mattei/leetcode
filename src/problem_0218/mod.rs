fn get_skyline(buildings: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut result = vec![];

    let mut lines = Vec::new();

    for building in buildings {
        lines.push((building[0], building[1], building[2]));
    }

    lines.sort_unstable_by(|l, r| r.0.cmp(&l.0));

    let (mut last_x1, mut last_x2, mut last_y) = (i32::MIN, i32::MIN, i32::MIN);

    while let Some((x1, x2, y)) = lines.pop() {
        // our building starts after last building
        if last_x2 < x1 {
            // line at 0 from previous end until ours
            result.push(vec![last_x2, 0]);
            // and start the new one
            result.push(vec![x1, y]);

            last_x1 = x1;
            last_x2 = x2;
            last_y = y;

            continue;
        }

        if last_x2 == x1 {
            // they touch
            if last_y == y {
                // extend
                last_x2 = x2;
                continue;
            }

            // jump up or down, new point, new start
            // and start the new one
            result.push(vec![x1, y]);

            last_x1 = x1;
            last_x2 = x2;
            last_y = y;

            continue;
        }

        // our building starts before the end of last building...
        // couple of cases

        // current building fits completely in last building -> ignore
        if y <= last_y && last_x2 >= x2 {
            continue;
        }

        // current building is lower but last building surpases our start -> our building will need to start at the last building's end
        if y < last_y {
            lines.push((last_x2, x2, y));
            lines.sort_unstable_by(|l, r| r.0.cmp(&l.0));
            continue;
        }

        if y == last_y {
            // extend
            last_x2 = x2;
            continue;
        }

        // current building is higher and our end surpases last building's start
        // current building is higher and but last buiding surpases our end -> last building will need to resume when ours ends
        if y > last_y {
            if x2 >= last_x2 {
                result.push(vec![last_x1, last_y]);

                if last_x2 == x1 {
                    // line at 0 from previous end until ours
                    result.push(vec![last_x2, 0]);
                }
                last_x1 = x1;
                last_x2 = x2;
                last_y = y;

                continue;
            }

            // split last building
            lines.push((last_x2, x2, y));
            lines.sort_unstable_by(|l, r| r.0.cmp(&l.0));
            continue;
        }

        unreachable!()
    }

    result.push(vec![last_x2, 0]);

    result[1..].to_vec()
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        get_skyline(&buildings)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0218::get_skyline;

    #[test]
    fn test_1() {
        assert_eq!(
            get_skyline(
                &[
                    [2, 9, 10],
                    [3, 7, 15],
                    [5, 12, 12],
                    [15, 20, 10],
                    [19, 24, 8]
                ]
                .iter()
                .map(Into::into)
                .collect::<Vec<Vec<i32>>>()
            ),
            [
                [2, 10],
                [3, 15],
                [7, 12],
                [12, 0],
                [15, 10],
                [20, 8],
                [24, 0]
            ]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            get_skyline(
                &[[0, 2, 3], [2, 5, 3]]
                    .iter()
                    .map(Into::into)
                    .collect::<Vec<Vec<i32>>>()
            ),
            [[0, 3], [5, 0]]
        );
    }
}
