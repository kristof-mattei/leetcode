use std::collections::VecDeque;

impl Solution {
    #[must_use]
    #[expect(clippy::needless_pass_by_value, reason = "External invocation")]
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        ladder_length(begin_word, &end_word, word_list)
    }
}

fn ladder_length(begin_word: String, end_word: &str, mut word_list: Vec<String>) -> i32 {
    let Some(end_word_index) = word_list.iter().position(|w| w == end_word) else {
        return 0;
    };

    word_list.push(begin_word);

    let neighbors = build_graph(&word_list);

    dijkstra(&neighbors, word_list.len() - 1, end_word_index)
        .map_or(0, |distance| distance + 1)
        .try_into()
        .unwrap()
}

fn build_graph(word_list: &[String]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; word_list.len()];

    for (from_word_index, from_word) in word_list.iter().enumerate() {
        for (to_word_index, to_word) in word_list.iter().enumerate() {
            if can_move(from_word, to_word) {
                graph[from_word_index].push(to_word_index);
            }
        }
    }

    graph
}

fn can_move(from_word: &str, to_word: &str) -> bool {
    let mut can_move = false;

    for (f, t) in from_word.as_bytes().iter().zip(to_word.as_bytes()) {
        if f != t {
            if can_move {
                return false;
            }

            can_move = true;
        }
    }

    can_move
}

fn dijkstra(neighbors: &[Vec<usize>], start: usize, end: usize) -> Option<usize> {
    let mut visited = vec![false; neighbors.len()];
    let mut distances = vec![usize::MAX; neighbors.len()];

    let mut queue = VecDeque::new();

    queue.push_back(start);

    distances[start] = 0;
    visited[start] = true;

    while let Some(node) = queue.pop_front() {
        let distance = distances[node] + 1;

        for neighbor in &neighbors[node] {
            if !visited[*neighbor] {
                let v = &mut distances[*neighbor];

                *v = (distance).min(*v);

                if neighbor == &end {
                    return Some(*v);
                }

                queue.push_back(*neighbor);
                visited[*neighbor] = true;
            }
        }
    }
    None
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0127::ladder_length;

    #[test]
    fn test_1() {
        assert_eq!(
            ladder_length(
                "hit".into(),
                "cog",
                ["hot", "dot", "dog", "lot", "log", "cog"]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
            ),
            5
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            ladder_length(
                "hit".into(),
                "cog",
                ["hot", "dot", "dog", "lot", "log"]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
            ),
            0
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            ladder_length(
                "hot".into(),
                "dog",
                ["hot", "cog", "dog", "tot", "hog", "hop", "pot", "dot"]
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
            ),
            3
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            ladder_length(
                "qa".into(),
                "sq",
                [
                    "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm",
                    "le", "av", "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo",
                    "ow", "sn", "ya", "cr", "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur",
                    "rh", "sr", "tc", "lt", "lo", "as", "fr", "nb", "yb", "if", "pb", "ge", "th",
                    "pm", "rb", "sh", "co", "ga", "li", "ha", "hz", "no", "bi", "di", "hi", "qa",
                    "pi", "os", "uh", "wm", "an", "me", "mo", "na", "la", "st", "er", "sc", "ne",
                    "mn", "mi", "am", "ex", "pt", "io", "be", "fm", "ta", "tb", "ni", "mr", "pa",
                    "he", "lr", "sq", "ye"
                ]
                .into_iter()
                .map(Into::into)
                .collect::<Vec<String>>()
            ),
            5
        );
    }
}
