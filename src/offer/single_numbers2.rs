struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut sum :i64 = 0;
        for i in nums{
            sum += i as i64;
            set.insert(i as i64);
        }
        ((set.iter().sum::<i64>()*3-sum)/2) as i32
    }

    fn single_number_by_mod(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..32{
            let mut count = 0;
            let x = 1<<i;
            for n in nums.clone(){
                if x & n == x{   //所有num的第i为都加起来 如果模3余1则该位为多的那个值的那一位
                    count+=1;
                }
            }
            if count%3 == 1{
                res+=x;
            }
        }
        res
    }
}