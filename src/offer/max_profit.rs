struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {return 0}
        let (mut res,mut mark) = (0,prices[0]);
        for i in prices{
            res = std::cmp::max(res,i-mark);
            mark = std::cmp::min(mark,i);
        }
        res
    }
}