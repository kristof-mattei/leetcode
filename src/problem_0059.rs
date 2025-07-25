enum Direction {
    Decrease,
    Standstill,
    Increase,
}

impl Direction {
    fn next(&self, u: usize) -> usize {
        match *self {
            Direction::Decrease => u.wrapping_sub(1),
            Direction::Standstill => u,
            Direction::Increase => u + 1,
        }
    }

    fn flip(&mut self) {
        *self = match *self {
            Direction::Decrease => Direction::Increase,
            Direction::Standstill => Direction::Standstill,
            Direction::Increase => Direction::Decrease,
        }
    }
}

fn generate_matrix(n: usize) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![vec![0; n]; n];

    let mut row: usize = 0;
    let mut col: usize = 0;

    let mut row_dir = Direction::Standstill;
    let mut column_dir = Direction::Increase;

    for k in 0..n * n {
        result[row][col] = (k + 1) as i32;

        // we start at 0, and if we find a non-0 we need to turn
        if result[(row_dir.next(row)) % n][(column_dir.next(col)) % n] > 0 {
            std::mem::swap(&mut row_dir, &mut column_dir);
            column_dir.flip();
        }

        row = row_dir.next(row);
        col = column_dir.next(col);
    }

    result
}

impl Solution {
    #[must_use]
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        generate_matrix(n as usize)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0059::generate_matrix;

    #[test]
    fn test_1() {
        assert_eq!(generate_matrix(3), [[1, 2, 3], [8, 9, 4], [7, 6, 5]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(generate_matrix(1), [[1]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            generate_matrix(6),
            [
                [1, 2, 3, 4, 5, 6],
                [20, 21, 22, 23, 24, 7],
                [19, 32, 33, 34, 25, 8],
                [18, 31, 36, 35, 26, 9],
                [17, 30, 29, 28, 27, 10],
                [16, 15, 14, 13, 12, 11]
            ]
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            generate_matrix(20),
            [
                [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
                ],
                [
                    76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 21
                ],
                [
                    75, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158,
                    159, 160, 95, 22
                ],
                [
                    74, 143, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217,
                    218, 161, 96, 23
                ],
                [
                    73, 142, 203, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268,
                    219, 162, 97, 24
                ],
                [
                    72, 141, 202, 255, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 269,
                    220, 163, 98, 25
                ],
                [
                    71, 140, 201, 254, 299, 336, 337, 338, 339, 340, 341, 342, 343, 344, 311, 270,
                    221, 164, 99, 26
                ],
                [
                    70, 139, 200, 253, 298, 335, 364, 365, 366, 367, 368, 369, 370, 345, 312, 271,
                    222, 165, 100, 27
                ],
                [
                    69, 138, 199, 252, 297, 334, 363, 384, 385, 386, 387, 388, 371, 346, 313, 272,
                    223, 166, 101, 28
                ],
                [
                    68, 137, 198, 251, 296, 333, 362, 383, 396, 397, 398, 389, 372, 347, 314, 273,
                    224, 167, 102, 29
                ],
                [
                    67, 136, 197, 250, 295, 332, 361, 382, 395, 400, 399, 390, 373, 348, 315, 274,
                    225, 168, 103, 30
                ],
                [
                    66, 135, 196, 249, 294, 331, 360, 381, 394, 393, 392, 391, 374, 349, 316, 275,
                    226, 169, 104, 31
                ],
                [
                    65, 134, 195, 248, 293, 330, 359, 380, 379, 378, 377, 376, 375, 350, 317, 276,
                    227, 170, 105, 32
                ],
                [
                    64, 133, 194, 247, 292, 329, 358, 357, 356, 355, 354, 353, 352, 351, 318, 277,
                    228, 171, 106, 33
                ],
                [
                    63, 132, 193, 246, 291, 328, 327, 326, 325, 324, 323, 322, 321, 320, 319, 278,
                    229, 172, 107, 34
                ],
                [
                    62, 131, 192, 245, 290, 289, 288, 287, 286, 285, 284, 283, 282, 281, 280, 279,
                    230, 173, 108, 35
                ],
                [
                    61, 130, 191, 244, 243, 242, 241, 240, 239, 238, 237, 236, 235, 234, 233, 232,
                    231, 174, 109, 36
                ],
                [
                    60, 129, 190, 189, 188, 187, 186, 185, 184, 183, 182, 181, 180, 179, 178, 177,
                    176, 175, 110, 37
                ],
                [
                    59, 128, 127, 126, 125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114,
                    113, 112, 111, 38
                ],
                [
                    58, 57, 56, 55, 54, 53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39
                ]
            ]
        );
    }
}
