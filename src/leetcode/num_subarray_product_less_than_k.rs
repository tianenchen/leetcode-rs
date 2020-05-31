struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 || k == 1 { return 0 }
        let mut head = 0;
        let mut product = 1;
        let mut res = 0;
        for tail in 0..nums.len() {
            product *= nums[tail];
            while product >= k {
                product /= nums[head];
                head += 1;
            }
            res += tail - head + 1;
        }
        res as i32
    }
    pub fn num_subarray_product_less_than_k2(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut res = 0;
        for i in 0..len {
            let mut product = 1;
            for j in 0..=i {
                product *= nums[i - j];
                if product >= k {
                    break;
                } else {
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
        8
    );
}
