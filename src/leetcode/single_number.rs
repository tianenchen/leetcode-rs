struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {

        // let mut s = String::new();

        // let mut sr = &mut s;//拿到了mut 引用

        // sr.push('a');

        // println!("aaaaa{}",&s); // 拿到了 引用

        // sr.push('a');//

        // let mut res = nums[0];
        // for i in &nums[1..]{
        //     res = res ^ i;
        // }
        // res
        let mut res = nums[0];
        for (i,r) in nums.iter().enumerate(){
            if i>0{
                res = res ^ r;
            }
        }
        res
    }
}

#[test]
fn check() {
    assert_eq!(Solution::single_number(vec![1,2,2,3,3,4,4]),1);
}