struct MinStack {
    inner: std::collections::LinkedList<i32>,
    mins: std::collections::LinkedList<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            inner: std::collections::LinkedList::default(),
            mins: std::collections::LinkedList::default(),
        }
    }

    fn push(&mut self, x: i32) {
        if let Some(min) = self.mins.back() {
            if x < *min {
                self.mins.push_back(x);
            } else {
                self.mins.push_back(*min);
            }
        } else {
            self.mins.push_back(x);
        }
        self.inner.push_back(x)
    }

    fn pop(&mut self) {
        self.inner.pop_back();
        self.mins.pop_back();
    }

    fn top(&self) -> i32 {
        *self.inner.back().unwrap()
    }

    fn min(&self) -> i32 {
        *self.mins.back().unwrap()
    }
}
