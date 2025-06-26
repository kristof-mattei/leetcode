#![expect(dead_code)]

#[derive(Debug)]
struct Node {
    tail: Option<Box<Node>>,
    value: i32,
}

impl Node {
    fn new(value: i32) -> Self {
        Self { tail: None, value }
    }
}

struct MyStack {
    last: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Self { last: None }
    }

    fn push(&mut self, x: i32) {
        let mut new_node = Node::new(x);

        let last = self.last.take();

        new_node.tail = last;

        self.last = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> i32 {
        let last = self.last.take().unwrap();

        let Node { value, tail } = *last;

        self.last = tail;

        value
    }

    fn top(&self) -> i32 {
        self.last.as_ref().unwrap().value
    }

    fn empty(&self) -> bool {
        self.last.is_none()
    }
}
