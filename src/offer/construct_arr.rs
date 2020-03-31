struct Solution;

impl Solution {
    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; a.len() - 1];
        let len = a.len();
        let mut tmp = 1;
        for i in 0..len {
            res[i] *= tmp;
            tmp *= a[i];
        }
        for i in (0..len).rev() {
            res[i] *= tmp;
            tmp *= a[i];
        }
        res
    }
}
