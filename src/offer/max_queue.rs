struct MaxQueue {
    queue : std::collections::VecDeque<i32>,
    max : std::collections::LinkedList<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MaxQueue {

    fn new() -> Self {
        MaxQueue{
            queue:std::collections::VecDeque::default(),
            max :std::collections::LinkedList::default()
        }
    }
    
    fn max_value(&self) -> i32 {
        if let Some(m) = self.max.front() {
            *m
        }
        else{
            -1
        }
    }
    
    fn push_back(&mut self, value: i32) {
        self.queue.push_back(value);
        while self.max.len()>0 && *self.max.back().unwrap() < value {
            self.max.pop_back();
        }
        self.max.push_back(value);
    }
    
    fn pop_front(&mut self) -> i32 {
        if let Some(v) = self.queue.pop_front() {
            if let Some(m) = self.max.front() {
                if v == *m {
                    self.max.pop_front();
                }
            }
            v
        }
        else{
            -1
        }
    }
}


#[test]
fn check() {
}