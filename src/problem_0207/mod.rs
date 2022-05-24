#[derive(Clone, PartialEq)]
enum State {
    NotVisited,
    InStack,
    Done,
}

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;

    let mut adjecent_list = vec![vec![]; num_courses];

    for p in prerequisites {
        adjecent_list[p[0] as usize].push(p[1] as usize);
    }

    let mut stack: Vec<usize> = Vec::with_capacity(num_courses);
    let mut states = vec![State::NotVisited; num_courses];

    #[allow(clippy::match_on_vec_items)]
    for v in 0..num_courses {
        if states[v] == State::NotVisited {
            stack.push(v);

            while let Some(v) = stack.pop() {
                match states[v] {
                    State::NotVisited => {
                        states[v] = State::InStack;
                        stack.push(v);

                        for &u in &adjecent_list[v] {
                            match states[u] {
                                State::InStack => {
                                    return false;
                                },
                                State::NotVisited => {
                                    stack.push(u);
                                },
                                State::Done => {},
                            }
                        }
                    },
                    State::InStack => {
                        states[v] = State::Done;
                    },
                    State::Done => {},
                };
            }
        }
    }

    true
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn can_finish(num_courses: i32, prerequisits: Vec<Vec<i32>>) -> bool {
        can_finish(num_courses, prerequisits)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0207::can_finish;
    #[test]
    fn test_1() {
        assert!(can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn test_2() {
        assert!(!can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }

    #[test]
    fn test_3() {
        assert!(can_finish(
            5,
            vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]
        ));
    }
}
