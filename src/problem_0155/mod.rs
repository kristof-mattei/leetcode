struct MinStack {
    stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.stack.iter().min().unwrap()
    }
}

impl Solution {
    pub fn problem_0155() {
        let mut min_stack = MinStack::new();

        min_stack.push(1);
        min_stack.pop();
        let _: i32 = min_stack.top();
        let _: i32 = min_stack.get_min();
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn test_1() {
        let mut min_stack = MinStack::new();

        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
