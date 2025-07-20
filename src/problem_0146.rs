struct Node {
    key: i32,
    value: i32,
    next: *mut Node,
    prev: *mut Node,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        }
    }
}

struct NodeList {
    first: Box<Node>,
    last: Box<Node>,
}

impl NodeList {
    fn new() -> Self {
        let mut nl = Self {
            first: Box::new(Node::new(0x0ead_beef, 0xead_beef)),
            last: Box::new(Node::new(0x0ead_beef, 0xead_beef)),
        };

        nl.first.next = nl.last.as_mut();
        nl.last.prev = nl.first.as_mut();

        nl
    }

    fn move_first(&mut self, node: &mut Node) {
        unsafe {
            (*node.next).prev = node.prev;
            (*node.prev).next = node.next;
        }
        self.push_first(node);
    }

    fn drop_last(&mut self) -> i32 {
        let last_key = unsafe { (*self.last.prev).key };
        unsafe {
            self.last.prev = (*self.last.prev).prev;
            (*self.last.prev).next = self.last.as_mut();
        }
        last_key
    }

    fn push_first(&mut self, node: &mut Node) {
        node.prev = self.first.as_mut();
        node.next = self.first.next;
        unsafe {
            (*self.first.next).prev = node;
        }
        self.first.next = node;
    }
}

struct LRUCache {
    map: std::collections::HashMap<i32, Box<Node>>,
    list: NodeList,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: std::collections::HashMap::with_capacity(capacity as usize),
            list: NodeList::new(),
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.map.entry(key) {
            let node = e.get_mut().as_mut();
            self.list.move_first(node);
            node.value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.map.entry(key) {
            let node = e.get_mut().as_mut();
            self.list.move_first(node);
            node.value = value;
        } else {
            if self.map.len() >= self.capacity {
                let last_key = self.list.drop_last();
                assert!(self.map.remove(&last_key).is_some());
            }
            let mut node = Box::new(Node::new(key, value));
            self.list.push_first(node.as_mut());
            self.map.insert(key, node);
        }
    }
}

impl Solution {
    #[must_use]
    pub fn problem_0146() -> i32 {
        let mut lru_cache = LRUCache::new(2);

        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        lru_cache.get(1)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0146::LRUCache;

    #[test]
    fn test_1() {
        let mut lru_cache = LRUCache::new(2);

        lru_cache.put(1, 1); // cache is {1=1}
        lru_cache.put(2, 2); // cache is {1=1, 2=2}
        assert_eq!(lru_cache.get(1), 1); // return 1
        lru_cache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        assert_eq!(lru_cache.get(2), -1); // returns -1 (not found)
        lru_cache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        assert_eq!(lru_cache.get(1), -1); // return -1 (not found)
        assert_eq!(lru_cache.get(3), 3); // return 3
        assert_eq!(lru_cache.get(4), 4); // return 4
    }

    #[test]
    fn test_2() {
        let mut lru_cache = LRUCache::new(2);

        assert_eq!(lru_cache.get(2), -1);
        lru_cache.put(2, 6);
        assert_eq!(lru_cache.get(1), -1);
        lru_cache.put(1, 5);
        lru_cache.put(1, 2);
        assert_eq!(lru_cache.get(1), 2);
        assert_eq!(lru_cache.get(2), 6);
    }
}
