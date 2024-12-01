use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct XY {
    x: i32,
    y: i32,
}

impl std::cmp::Ord for XY {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl std::cmp::PartialOrd for XY {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Building {
    start: i32,
    end: i32,
    height: i32,
}

impl Building {
    fn new(start: i32, end: i32, height: i32) -> Self {
        Self { start, end, height }
    }
}

fn get_skyline(raw_buildings: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let buildings = raw_buildings
        .iter()
        .map(|v| Building::new(v[0], v[1], v[2]))
        .collect::<Vec<_>>();

    let size = buildings.len();

    let mut result = Vec::<_>::with_capacity(size);

    let mut heap = BinaryHeap::<XY>::with_capacity(size);

    let mut x = buildings[0].start;

    let mut i = 0;

    while i < size || !heap.is_empty() {
        while i < size && buildings[i].start <= x {
            heap.push(XY {
                x: buildings[i].end,
                y: buildings[i].height,
            });

            i += 1;
        }

        while let Some(v) = heap
            .peek_mut()
            .and_then(|v| if v.x <= x { Some(v) } else { None })
        {
            PeekMut::pop(v);
        }

        let y = heap.peek().map_or(0, |t| t.y);

        result.push(vec![x, y]);

        x = *[heap.peek().map(|t| t.x), buildings.get(i).map(|b| b.start)]
            .iter()
            .flatten()
            .min()
            .unwrap_or(&x);
    }

    let mut h = Some(0);

    result.retain(|a| h.replace(a[1]).unwrap() != a[1]);

    result
}

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value)]
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
                [24, 0],
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

    #[test]
    fn test_3() {
        assert_eq!(
            get_skyline(
                &[
                    [4, 9, 10],
                    [4, 9, 15],
                    [4, 9, 12],
                    [10, 12, 10],
                    [10, 12, 8]
                ]
                .iter()
                .map(Into::into)
                .collect::<Vec<Vec<i32>>>()
            ),
            [[4, 15], [9, 0], [10, 10], [12, 0]]
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            get_skyline(
                &[[2, 13, 10], [10, 17, 25], [12, 20, 14]]
                    .iter()
                    .map(Into::into)
                    .collect::<Vec<Vec<i32>>>()
            ),
            [[2, 10], [10, 25], [17, 14], [20, 0]]
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            get_skyline(
                &[
                    [3, 7, 8],
                    [3, 8, 7],
                    [3, 9, 6],
                    [3, 10, 5],
                    [3, 11, 4],
                    [3, 12, 3],
                    [3, 13, 2],
                    [3, 14, 1]
                ]
                .iter()
                .map(Into::into)
                .collect::<Vec<Vec<i32>>>()
            ),
            [
                [3, 8],
                [7, 7],
                [8, 6],
                [9, 5],
                [10, 4],
                [11, 3],
                [12, 2],
                [13, 1],
                [14, 0]
            ]
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            get_skyline(
                &[
                    [1, 38, 219],
                    [2, 19, 228],
                    [2, 64, 106],
                    [3, 80, 65],
                    [3, 84, 8],
                    [4, 12, 8],
                    [4, 25, 14],
                    [4, 46, 225],
                    [4, 67, 187],
                    [5, 36, 118],
                    [5, 48, 211],
                    [5, 55, 97],
                    [6, 42, 92],
                    [6, 56, 188],
                    [7, 37, 42],
                    [7, 49, 78],
                    [7, 84, 163],
                    [8, 44, 212],
                    [9, 42, 125],
                    [9, 85, 200],
                    [9, 100, 74],
                    [10, 13, 58],
                    [11, 30, 179],
                    [12, 32, 215],
                    [12, 33, 161],
                    [12, 61, 198],
                    [13, 38, 48],
                    [13, 65, 222],
                    [14, 22, 1],
                    [15, 70, 222],
                    [16, 19, 196],
                    [16, 24, 142],
                    [16, 25, 176],
                    [16, 57, 114],
                    [18, 45, 1],
                    [19, 79, 149],
                    [20, 33, 53],
                    [21, 29, 41],
                    [23, 77, 43],
                    [24, 41, 75],
                    [24, 94, 20],
                    [27, 63, 2],
                    [31, 69, 58],
                    [31, 88, 123],
                    [31, 88, 146],
                    [33, 61, 27],
                    [35, 62, 190],
                    [35, 81, 116],
                    [37, 97, 81],
                    [38, 78, 99],
                    [39, 51, 125],
                    [39, 98, 144],
                    [40, 95, 4],
                    [45, 89, 229],
                    [47, 49, 10],
                    [47, 99, 152],
                    [48, 67, 69],
                    [48, 72, 1],
                    [49, 73, 204],
                    [49, 77, 117],
                    [50, 61, 174],
                    [50, 76, 147],
                    [52, 64, 4],
                    [52, 89, 84],
                    [54, 70, 201],
                    [57, 76, 47],
                    [58, 61, 215],
                    [58, 98, 57],
                    [61, 95, 190],
                    [66, 71, 34],
                    [66, 99, 53],
                    [67, 74, 9],
                    [68, 97, 175],
                    [70, 88, 131],
                    [74, 77, 155],
                    [74, 99, 145],
                    [76, 88, 26],
                    [82, 87, 40],
                    [83, 84, 132],
                    [88, 99, 99]
                ]
                .iter()
                .map(Into::into)
                .collect::<Vec<Vec<i32>>>()
            ),
            [
                [1, 219],
                [2, 228],
                [19, 225],
                [45, 229],
                [89, 190],
                [95, 175],
                [97, 152],
                [99, 74],
                [100, 0]
            ]
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(
            get_skyline(
                &[
                    [1, 2, 1],
                    [1, 2, 2],
                    [1, 2, 3],
                    [2, 3, 1],
                    [2, 3, 2],
                    [2, 3, 3]
                ]
                .iter()
                .map(Into::into)
                .collect::<Vec<Vec<i32>>>()
            ),
            [[1, 3], [3, 0]]
        );
    }
}
