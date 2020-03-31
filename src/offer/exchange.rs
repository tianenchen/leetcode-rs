struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0_i32; nums.len()];
        let (mut head, mut tail) = (0, nums.len() - 1);

        for i in nums.into_iter() {
            if i % 2 == 0 {
                res[tail] = i;
                tail -= 1;
            } else {
                res[head] = i;
                head += 1;
            }
        }
        res
    }
}

#[test]
fn check() {
    assert_eq!(Solution::exchange(vec![1, 2, 3, 4, 5]), vec![1, 3, 5, 4, 2]);
}
