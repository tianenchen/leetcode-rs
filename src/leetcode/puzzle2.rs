#[derive(Debug)]
struct Change {
    //array数组，直接定义数组的话由于编译时编译器不知道结构体对象的具体大小所以包了一个Box对象
    array: Box<[i32]>,
    n: usize,
}

//impl可以简单理解为Change对象的成员函数
impl Change {
    fn new(array: Box<[i32]>, n: usize) -> Self {
        Change { array, n }
    }

    //交换函数，具体逻辑就是一次遍历遍历途中记录最大最小值以及索引，然后在最后交换他们
    fn exchange(&mut self) -> f64 {
        if self.n > 0 {
            let mut sum = 0;
            let (mut max, mut min) = (self.array[0], self.array[0]);
            let (mut max_idx, mut min_idx) = (0, 0);
            for i in 0..self.n {
                if self.array[i] < min {
                    min = self.array[i];
                    min_idx = i;
                }
                if self.array[i] > max {
                    max = self.array[i];
                    max_idx = i;
                }
                sum += self.array[i];
            }
            //交换逻辑
            let tmp = self.array[max_idx];
            self.array[max_idx] = self.array[min_idx];
            self.array[min_idx] = tmp;
            //这里as f64相当于将int32转为double
            return sum as f64 / self.n as f64;
        }
        0_f64
    }

    fn print(&self) {
        println!("{:?}", self.array);
    }
}

//这是一个test方法去，编译时会跳过
#[test]
fn check() {
    let mut change = Change::new(Box::new([21, 65, 43, 87, 12, 84, 44, 97, 32, 55]), 10);
    // change.print();
    let avg = change.exchange();
    // dbg!(avg);
    // change.print()

    /* console display:
        [21, 65, 43, 87, 12, 84, 44, 97, 32, 55]
        [src/leetcode/puzzle2.rs:52] avg = 54.0
        [21, 65, 43, 87, 97, 84, 44, 12, 32, 55]
    */
}
