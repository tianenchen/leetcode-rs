pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for (i,t) in prices.iter().enumerate(){
            match prices.get(i+1){
                Some(v) if v > t => {profit+=v-t},
                _=>(),
            }
        }
        return profit
    }
}

#[test]
fn check() {
    assert_eq!(Solution::max_profit(vec![1,2,3,4,5,6,7]),6);
    assert_eq!(Solution::max_profit(vec![7,6,5,4,3,2,1]),0);
    assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]),7);
}