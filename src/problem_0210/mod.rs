#[derive(Clone, PartialEq)]
enum State {
    NotVisited,
    InStack,
    Done,
}

fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;

    let mut adjecent_list = vec![vec![]; num_courses];

    for p in prerequisites {
        adjecent_list[p[0] as usize].push(p[1] as usize);
    }

    let mut stack: Vec<usize> = Vec::with_capacity(num_courses);
    let mut states = vec![State::NotVisited; num_courses];

    let mut order = Vec::with_capacity(num_courses);

    #[allow(clippy::match_on_vec_items)]
    // courses go from 0 to num_courses - 1
    for v in 0..num_courses {
        if states[v] == State::NotVisited {
            stack.push(v);

            // fetch the next cource
            while let Some(v) = stack.pop() {
                match states[v] {
                    | State::NotVisited => {
                        // We put it back IN thet stack
                        states[v] = State::InStack;
                        stack.push(v);

                        for &u in &adjecent_list[v] {
                            match states[u] {
                                | State::InStack => {
                                    // to do the current task (v)
                                    // we need to do u, but u is dependent on us
                                    // so we are stuck
                                    return vec![];
                                },
                                | State::NotVisited => {
                                    // we didn't do the prerequisite yet, let's try and process it
                                    stack.push(u);
                                },
                                | State::Done => {
                                    // Great, we did the prerequisite
                                },
                            }
                        }
                    },

                    | State::InStack => {
                        // this means that we went from
                        // NotVisited to a whole set of dependencies
                        // back to us
                        // because of that we know that the dependencies
                        // have been resolved, and we can solve the chain of `v`
                        states[v] = State::Done;

                        // and since it's done, this is the order we're supposed to complete it =)
                        order.push(v as i32);
                    },
                    | State::Done => {
                        // we already did it, ignore
                    },
                };
            }
        }
    }

    order
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_order(num_courses: i32, prerequisits: Vec<Vec<i32>>) -> Vec<i32> {
        find_order(num_courses, prerequisits)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0210::find_order;
    #[test]
    fn test_1() {
        assert_eq!(find_order(2, vec![vec![1, 0]]), [0, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            [0, 1, 2, 3]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(find_order(1, vec![]), [0]);
    }
}
