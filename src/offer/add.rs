struct Solution;

impl Solution {
    pub fn add(mut a: i32,mut b: i32) -> i32 {
        while a != 0{
            let tmp = a ^ b;
            b = (a & b) << 1;
            a = tmp;
        }
        a
    }
}