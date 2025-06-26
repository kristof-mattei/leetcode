#![expect(dead_code)]

struct MedianFinder {
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder { v: vec![] }
    }

    fn add_num(&mut self, num: i32) {
        let i = match self.v.binary_search(&num) {
            Ok(i) | Err(i) => i,
        };

        self.v.insert(i, num);
    }

    fn find_median(&self) -> f64 {
        let len_divided_by_2 = self.v.len() / 2;
        if self.v.len() % 2 == 0 {
            (f64::from(self.v[len_divided_by_2 - 1]) + f64::from(self.v[len_divided_by_2])) / 2f64
        } else {
            f64::from(self.v[len_divided_by_2])
        }
    }
}
