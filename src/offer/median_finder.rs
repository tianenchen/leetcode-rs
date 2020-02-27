use std::cmp::Reverse;

struct MedianFinder {
    count:usize,
    min_heap: std::collections::BinaryHeap<Reverse<i32>> , //小堆存中位数大于
    max_heap: std::collections::BinaryHeap<i32>,//大堆存小
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */


/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder{
            count:0,
            min_heap:std::collections::BinaryHeap::default(),
            max_heap:std::collections::BinaryHeap::default(),
        }
    }
    //max_heap    num    min_heap
    fn add_num(&mut self, num: i32){
        if self.min_heap.len() == self.max_heap.len(){
            if let Some(mut max) = self.max_heap.peek_mut() {
                if num < *max {
                    let tmp = *max;
                    *max = num;
                    self.min_heap.push(Reverse(tmp));
                }
                else{
                    self.min_heap.push(Reverse(num));
                }
            }
            else{
                self.min_heap.push(Reverse(num));
            }
        }
        else{
            if let Some(mut min) = self.min_heap.peek_mut() {
                if num>min.0{
                    let tmp = min.0;
                    *min = Reverse(num);
                    self.max_heap.push(tmp);
                }
                else{
                    self.max_heap.push(num);
                }
            }
        }
        self.count+=1;
    }
    
    fn find_median(&self) -> f64 {
        if self.count == 0 {
            0_f64
        }
        else if self.count%2==0{
            (self.min_heap.peek().unwrap().0+self.max_heap.peek().unwrap()) as f64 / 2_f64
        }
        else{
            self.min_heap.peek().unwrap().0 as f64
        }
    }
}
