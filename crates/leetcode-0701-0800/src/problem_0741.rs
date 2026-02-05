use std::cmp::PartialEq;
use std::collections::{BinaryHeap, HashMap};
use std::ops::{Index, IndexMut};

trait IntoVecVec<T> {
    fn into_vec_vec(self) -> Vec<Vec<T>>;
}

impl<T, const R: usize, const C: usize> IntoVecVec<T> for [[T; C]; R] {
    fn into_vec_vec(self) -> Vec<Vec<T>> {
        self.into_iter()
            .map(|row| row.into_iter().collect::<Vec<T>>())
            .collect()
    }
}

impl<T> IntoVecVec<T> for Vec<Vec<T>> {
    fn into_vec_vec(self) -> Vec<Vec<T>> {
        self
    }
}

struct Grid<T> {
    data: Vec<Row<T>>,
    row_len: usize,
    column_len: usize,
    // max_row: usize,
    // max_column: usize,
}

impl<T> Grid<T> {
    /// Builds a new grid.
    ///
    /// # Panics
    /// When rows are not equal length.
    #[must_use]
    pub fn new<I>(data: I) -> Self
    where
        I: IntoVecVec<T>,
    {
        let data = data.into_vec_vec();

        for w in data.windows(2) {
            assert_eq!(w[0].len(), w[1].len());
        }

        let rows = data.len();
        let columns = data[0].len();

        Self {
            data: data.into_iter().map(|r| Row(r)).collect(),
            row_len: rows,
            column_len: columns,
            // max_row: rows - 1,
            // max_column: columns - 1,
        }
    }

    fn get_row_length(&self) -> usize {
        self.row_len
    }

    fn get_column_length(&self) -> usize {
        self.column_len
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Row<T>(Vec<T>);

impl<T> Index<usize> for Row<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Row<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Row<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Cherry,
    Thorn,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Coordinates {
    row_index: usize,
    column_index: usize,
}

impl From<(usize, usize)> for Coordinates {
    fn from((row_index, column_index): (usize, usize)) -> Self {
        Coordinates {
            row_index,
            column_index,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn apply<T>(self, grid: &Grid<T>, row_column_index: Coordinates) -> Option<Coordinates> {
        let row_limit = grid.get_row_length();
        let column_limit = grid.get_column_length();

        let Coordinates {
            row_index,
            column_index,
        } = row_column_index;

        match self {
            Direction::North => (row_index.checked_sub(1)).map(|up| (up, column_index).into()),
            Direction::East => {
                let right = column_index + 1;
                if right < column_limit {
                    Some((row_index, right).into())
                } else {
                    None
                }
            },
            Direction::South => {
                let down = row_index + 1;

                if down < row_limit {
                    Some((down, column_index).into())
                } else {
                    None
                }
            },
            Direction::West => (column_index.checked_sub(1)).map(|left| (row_index, left).into()),
        }
    }
}

fn get_neighbor_directions<const D: usize>(
    grid: &Grid<Cell>,
    coordinates: Coordinates,
    directions: [Direction; D],
) -> [Option<Coordinates>; D] {
    let mut neighbors: [Option<Coordinates>; D] = [None; D];

    for i in 0..D {
        let direction = directions[i];
        if let Some(new_row_column_index) = direction.apply(grid, coordinates) {
            let neighbor = grid[new_row_column_index.row_index][new_row_column_index.column_index];

            if neighbor == Cell::Empty || neighbor == Cell::Cherry {
                neighbors[i] = Some(new_row_column_index);
            }
        }
    }

    neighbors
}

#[derive(Eq, Copy, Clone)]
struct Node {
    coordinates: Coordinates,
    berries_plucked: u32,
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coordinates.hash(state);
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.berries_plucked.cmp(&other.berries_plucked)
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn distance(grid: &Grid<Cell>, _current: Coordinates, neighbord: Coordinates) -> u32 {
    u32::from(grid[neighbord.row_index][neighbord.column_index] == Cell::Cherry)
}

#[expect(clippy::cast_possible_truncation)]
fn heuristic(_grid: &Grid<Cell>, current: Coordinates, goal: Coordinates) -> u32 {
    (current.row_index.abs_diff(goal.row_index) + current.column_index.abs_diff(goal.column_index))
        as u32
}

fn reconstruct_path<'l1, 'l2>(
    came_from: &'l1 HashMap<Node, Node>,
    mut current: &'l2 Node,
) -> Vec<Coordinates>
where
    'l1: 'l2,
{
    let mut total_path = vec![current.coordinates];

    while let Some(next) = came_from.get(current) {
        total_path.push(next.coordinates);

        current = next;
    }

    total_path.reverse();

    total_path
}
fn a_star<const D: usize>(
    grid: &Grid<Cell>,
    start: Coordinates,
    goal: Coordinates,
    directions: [Direction; D],
) -> Option<(HashMap<Node, Node>, Node)> {
    let start = Node {
        coordinates: start,
        berries_plucked: 0,
    };

    let mut came_from = HashMap::<Node, Node>::new();

    let mut g_score: HashMap<Node, u32> = HashMap::from([(start, 0)]);

    // we don't keep an f_score as it is incoorporated in the Node
    let mut open_set = BinaryHeap::from([start]);

    while let Some(node) = open_set.pop() {
        if node.coordinates == goal {
            return Some((came_from, node));
        }

        let neighbors = get_neighbor_directions(grid, node.coordinates, directions);

        for neighbor in neighbors.iter().filter_map(|&x| x) {
            let tentative_g_score =
                g_score.get(&node).unwrap() + distance(grid, node.coordinates, neighbor);

            let f_score = tentative_g_score + heuristic(grid, node.coordinates, goal);

            let neighbor_node = Node {
                coordinates: neighbor,
                berries_plucked: f_score,
            };

            let is_better = g_score
                .get(&neighbor_node)
                .is_none_or(|&s| tentative_g_score > s);

            if is_better {
                g_score.insert(neighbor_node, tentative_g_score);
                came_from.insert(neighbor_node, node);
                // we _could_ check if the open_set already contains the neighbor node, but
                // this is so costly that it's easier to insert with a higher cost and then it'll get either ignored
                // or discarded when the score is considered too high
                open_set.push(neighbor_node);
            }
        }
    }

    None
}

impl Solution {
    #[must_use]
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        cherry_pickup(grid)
    }
}

fn cherry_pickup(input: Vec<Vec<i32>>) -> i32 {
    let input: Vec<Vec<Cell>> = input
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| match cell {
                    -1 => Cell::Thorn,
                    0 => Cell::Empty,
                    1 => Cell::Cherry,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut grid = Grid::new(input);

    if grid.get_row_length() == 1 && grid.get_column_length() == 1 && grid[0][0] == Cell::Cherry {
        return 1;
    }

    let Some((taken, last)) = a_star(
        &grid,
        (0, 0).into(),
        (grid.get_row_length() - 1, grid.get_column_length() - 1).into(),
        [Direction::East, Direction::South],
    ) else {
        return 0;
    };

    let mut going_cherries_plucked = 0;

    for p in reconstruct_path(&taken, &last) {
        if grid[p.row_index][p.column_index] == Cell::Cherry {
            going_cherries_plucked += 1;
            grid[p.row_index][p.column_index] = Cell::Empty;
        }
    }

    let (_, last) = a_star(
        &grid,
        last.coordinates,
        (0, 0).into(),
        [Direction::North, Direction::West],
    )
    .unwrap();

    (going_cherries_plucked + last.berries_plucked) as i32
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0741::cherry_pickup;

    #[test]
    fn test_1() {
        assert_eq!(
            cherry_pickup(
                [[0, 1, -1], [1, 0, -1], [1, 1, 1]]
                    .into_iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
            ),
            5
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            cherry_pickup(
                [[1, 1, -1], [1, -1, 1], [-1, 1, 1]]
                    .into_iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
            ),
            0
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            cherry_pickup(
                [[1]]
                    .into_iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
            ),
            1
        );
    }

    #[test]
    fn test_4() {
        let input = [
            [1, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 1],
            [1, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1],
        ];

        assert_eq!(
            cherry_pickup(
                input
                    .into_iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
            ),
            15
        );
    }
}
