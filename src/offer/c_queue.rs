
use std::collections::LinkedList;

struct Cqueue {
    pos: LinkedList<i32>,
    neg:LinkedList<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl Cqueue {

    fn new() -> Self {
        Cqueue{
            pos:LinkedList::new(),
            neg:LinkedList::new(),
        }
    }
    
    fn append_tail(&mut self, value: i32) {
        self.pos.push_back(value);
    }
    
    fn delete_head(&mut self) -> i32 {
        if !self.neg.is_empty() {
            self.neg.pop_back().unwrap()
        }
        else if self.pos.is_empty(){
            -1
        }
        else{
            while let Some(v) = self.pos.pop_back() {
                self.neg.push_back(v);
            }
            self.neg.pop_back().unwrap()
        }
    }
}
