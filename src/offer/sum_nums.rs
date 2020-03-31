struct Solution;

impl Solution {
    pub fn sum_nums(mut n: i32) -> i32 {
        let mut res = 0;
        Self::help(&mut res, n);
        res
    }
    fn help(res: &mut i32, mut n: i32) -> bool {
        *res += n;
        n -= 1;
        n > 0 && Self::help(res, n)
    }
}
