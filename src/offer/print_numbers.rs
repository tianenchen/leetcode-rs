struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        let len = 10_i32.pow(n as u32) as usize - 1;
        let mut res = Vec::with_capacity(len);
        for i in 0..len as i32 {
            res.push(i + 1);
        }
        res
    }
}
