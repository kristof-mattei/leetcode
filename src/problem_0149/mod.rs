use std::collections::HashMap;

fn greatest_common_divisor(mut x: i32, mut y: i32) -> i32 {
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }

    let mut r;

    while y != 0 {
        r = x % y;
        x = y;
        y = r;
    }
    x
}

fn get_slope(x: i32, y: i32) -> (i32, i32) {
    if x == 0 {
        (0, i32::MAX)
    } else if y == 0 {
        (i32::MAX, 0)
    } else {
        let g = greatest_common_divisor(x.abs(), y.abs());

        if x < 0 {
            (-x / g, -y / g)
        } else {
            (x / g, y / g)
        }
    }
}

fn max_points(points: &[Vec<i32>]) -> i32 {
    if points.len() == 1 {
        return 1;
    }

    let mut frequency = HashMap::new();
    let mut max_points = 0;

    for (i, r) in points.iter().enumerate() {
        if points.len() - i <= max_points {
            break;
        }

        for l in points.iter().skip(i + 1) {
            // this shortcut doesnt't win anything
            // for (j, l) in points.iter().skip(i + 1).enumerate() {
            // if (points.len() - j) + frequency.values().max().unwrap_or(&0usize) <= max_points {
            //     break;
            // }

            let slope = get_slope(r[0] - l[0], r[1] - l[1]);
            let cur = frequency.entry(slope).and_modify(|c| *c += 1).or_insert(2);

            max_points = usize::max(max_points, *cur);
        }

        frequency.clear();
    }

    max_points as i32
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        max_points(&points)
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use crate::problem_0149::max_points;

    #[test]
    fn test_1() {
        assert_eq!(max_points(&[vec![1, 1], vec![2, 2], vec![3, 3]]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            max_points(&[
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ]),
            4
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(max_points(&[vec![0, 1], vec![0, 0]]), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            max_points(&[
                vec![-184, -551],
                vec![-105, -467],
                vec![-90, -394],
                vec![-60, -248],
                vec![115, 359],
                vec![138, 429],
                vec![60, 336],
                vec![150, 774],
                vec![207, 639],
                vec![-150, -686],
                vec![-135, -613],
                vec![92, 289],
                vec![23, 79],
                vec![135, 701],
                vec![0, 9],
                vec![-230, -691],
                vec![-115, -341],
                vec![-161, -481],
                vec![230, 709],
                vec![-30, -102]
            ]),
            11
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(max_points(&[vec![0, 0], vec![1, -1], vec![1, 1]]), 2);
    }

    #[test]
    fn test_6() {
        assert_eq!(max_points(&[vec![0, 0], vec![1, 1], vec![2, 2]]), 3);
    }

    #[test]
    fn test_7() {
        assert_eq!(max_points(&[vec![2, 0], vec![3, 9], vec![4, 34]]), 2);
    }

    #[test]
    fn test_8() {
        assert_eq!(
            max_points(&[
                vec![4, -3],
                vec![970, 680],
                vec![-97, -35],
                vec![3, 8],
                vec![60, 253],
                vec![0, -13],
                vec![-270, -748],
                vec![-291, -165],
                vec![270, 890],
                vec![90, 228],
                vec![-220, -270],
                vec![-255, -118],
                vec![873, 615],
                vec![-42, -175],
                vec![440, 345],
                vec![4, -9],
                vec![170, 27],
                vec![425, 114],
                vec![56, 203],
                vec![531, 872],
                vec![295, 480],
                vec![231, 193],
                vec![291, 225],
                vec![680, 201],
                vec![-10, 9],
                vec![-388, -230],
                vec![-385, -127],
                vec![-590, -990],
                vec![-7, -40],
                vec![308, 222],
                vec![-616, -247],
                vec![-70, -283],
                vec![150, 526],
                vec![77, 113],
                vec![396, 304],
                vec![-264, -311],
                vec![-6, -8],
                vec![-88, -147],
                vec![30, 162],
                vec![49, 176],
                vec![81, 196],
                vec![-9, -124],
                vec![-27, -188],
                vec![-14, -67],
                vec![308, 233],
                vec![413, 676],
                vec![-77, 33],
                vec![-177, -304],
                vec![0, -31],
                vec![472, 774],
                vec![462, 313],
                vec![-35, -148],
                vec![1, -2],
                vec![-440, -475],
                vec![154, 153],
                vec![485, 355],
                vec![-231, -47],
                vec![340, 85],
                vec![-60, -111],
                vec![42, 149],
                vec![-354, -598],
                vec![388, 290],
                vec![44, -24],
                vec![3, -8],
                vec![510, 143],
                vec![-308, -352],
                vec![-18, -156],
                vec![-21, -94],
                vec![-63, -316],
                vec![-118, -206],
                vec![0, 73],
                vec![-240, -657],
                vec![-352, -393],
                vec![-531, -892],
                vec![-485, -295],
                vec![352, 263],
                vec![616, 393],
                vec![-154, -7],
                vec![3, 4],
                vec![-5, -9],
                vec![63, 230],
                vec![385, 273],
                vec![-679, -425],
                vec![-595, -234],
                vec![-582, -360],
                vec![-176, -229],
                vec![770, 473],
                vec![-539, -207],
                vec![-56, -229],
                vec![-236, -402],
                vec![-970, -620],
                vec![-425, -176],
                vec![240, 799],
                vec![118, 186],
                vec![10, -7],
                vec![-680, -263],
                vec![-5, 7],
                vec![220, 140],
                vec![-2, 7],
                vec![-28, -121],
                vec![-300, -839],
                vec![-54, -284],
                vec![-194, -100],
                vec![-308, -87],
                vec![-3, -10],
                vec![-873, -555],
                vec![-90, -202],
                vec![-5, -4]
            ]),
            16
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(max_points(&[vec![4, 5], vec![4, -1], vec![4, 0]]), 3);
    }
}
