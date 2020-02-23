struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut counter = 0;
        while n > 0{
            if n & 1 == 1{
                counter+=1;
            }
            n >>= 1;
        }
        counter as i32
    }
}

#[test]
fn check() {

    assert_eq!(Solution::hamming_weight(9),2);
}