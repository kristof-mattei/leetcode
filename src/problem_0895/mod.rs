use std::collections::HashMap;

struct FreqStack {
    frequency_map: HashMap<i32, i32>,
    set_map: HashMap<i32, Vec<i32>>,
    maximum_frequency: i32,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            frequency_map: HashMap::new(),
            set_map: HashMap::new(),
            maximum_frequency: 0,
        }
    }

    fn push(&mut self, x: i32) {
        let frequency = *self
            .frequency_map
            .entry(x)
            .and_modify(|v| *v += 1)
            .or_insert(1);

        self.maximum_frequency = self.maximum_frequency.max(frequency);
        self.set_map
            .entry(frequency)
            .and_modify(|v| v.push(x))
            .or_insert_with(|| vec![x]);
    }

    fn pop(&mut self) -> i32 {
        let value = self.set_map.get_mut(&self.maximum_frequency).unwrap();

        // pop guaranteed by exercise
        let top = value.pop().unwrap();

        let _: std::collections::hash_map::Entry<i32, i32> =
            self.frequency_map.entry(top).and_modify(|v| *v -= 1);

        if value.is_empty() {
            self.maximum_frequency -= 1;
        }
        top
    }
}

impl Solution {
    #[must_use]
    pub fn problem_0895() -> i32 {
        let mut f = FreqStack::new();

        f.push(5);

        f.pop()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::FreqStack;

    #[test]
    fn test_1() {
        let mut f = FreqStack::new();
        f.push(5); // The stack is [5]
        f.push(7); // The stack is [5,7]
        f.push(5); // The stack is [5,7,5]
        f.push(7); // The stack is [5,7,5,7]
        f.push(4); // The stack is [5,7,5,7,4]
        f.push(5); // The stack is [5,7,5,7,4,5]
        assert_eq!(f.pop(), 5); // return 5, as 5 is the most frequent. The stack becomes [5,7,5,7,4].
        assert_eq!(f.pop(), 7); // return 7, as 5 and 7 is the most frequent, but 7 is closest to the top. The stack becomes [5,7,5,4].
        assert_eq!(f.pop(), 5); // return 5, as 5 is the most frequent. The stack becomes [5,7,4].
        assert_eq!(f.pop(), 4); // return 4, as 4, 5 and 7 is the most frequent, but 4 is closest to the top. The stack becomes [5,7].
    }
}
