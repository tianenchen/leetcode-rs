struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut max_times = 0;
        let mut max_key = 0;
        for i in nums.into_iter(){
            if let Some(v) = map.get_mut(&i) {
                *v+=1;
            }
            else{
                map.insert(i,1);
            }
            if let Some(v) = map.get(&i) {
                if *v > max_times{
                    max_times = *v;
                    max_key = i;
                }
            }
        }
        max_key
    }
}