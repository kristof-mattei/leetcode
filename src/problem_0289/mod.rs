impl Solution {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn game_of_life(input: &mut Vec<Vec<i32>>) {
        game_of_life(input);
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), // top-left
    (-1, 0),  // top
    (-1, 1),  // top-right
    (0, -1),  // left
    (0, 1),   // right
    (1, -1),  // bottom-left
    (1, 0),   // bottom
    (1, 1),   // bottom-right
];

fn count_neighbors(matrix: &[Vec<i32>], coordinates: (usize, usize)) -> usize {
    let mut count = 0;

    for direction in DIRECTIONS {
        let new_row = coordinates.0.checked_add_signed(direction.0);
        let new_column = coordinates.1.checked_add_signed(direction.1);

        if let Some((new_row, new_column)) = new_row.zip(new_column) {
            if matrix
                .get(new_row)
                .and_then(|row| row.get(new_column))
                .map(|v| without_flag(*v))
                == Some(1)
            {
                count += 1;
            }
        }
    }

    count
}

fn set_flag(i: &mut i32) {
    let bit_shifted = (1u32 << 31) as i32;
    *i ^= bit_shifted;
}

fn get_flag(i: i32) -> u32 {
    let unshifted_flag = i as u32 & (1 << 31);
    unshifted_flag >> 31
}

fn without_flag(i: i32) -> i32 {
    ((i as u32) & !(1u32 << 31)) as i32
}

fn game_of_life(input: &mut [Vec<i32>]) {
    for row_index in 0..input.len() {
        for column_index in 0..input[row_index].len() {
            let neighbors = count_neighbors(input, (row_index, column_index));

            let flag = match (without_flag(input[row_index][column_index]), neighbors) {
                (1, 2 | 3) | (0, 3) => 1,
                _ => 0,
            };

            if flag == 1 {
                set_flag(&mut input[row_index][column_index]);
            }
        }
    }

    for row in input {
        for cell in row {
            let flag = get_flag(*cell);

            if flag == 1 {
                *cell = 1;
            } else {
                *cell = 0;
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0289::{game_of_life, get_flag, set_flag, without_flag};

    #[test]
    fn test_1() {
        let mut v = 0;

        set_flag(&mut v);

        assert_eq!(without_flag(v), 0);
    }

    #[test]
    fn test_2() {
        let mut v = 0;

        set_flag(&mut v);

        assert_eq!(get_flag(v), 1);
    }

    #[test]
    fn test_3() {
        let mut input = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];

        game_of_life(&mut input);

        assert_eq!(input, [[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]]);
    }
}
