pub struct Solution;

impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut start = {
            if target % 2 == 0{ target/2-2 }// 偶数  从 中间往前两位开始计算
            else{ target/2 }
        };
        let mut end = start + 1;
        let mut res = vec![];
        while start > 0 && start < end {
            let jug = (start + end) * (end-start + 1) / 2;
            if jug == target{
                res.push((start..=end).collect::<Vec<i32>>());
                start-=2;
                end -=1;
            }
            else if jug > target {
                end -=1;
            }
            else{
                start -=1;
            }
        }
        res.reverse();
        res
    }
}

#[test]
fn check() {
    assert_eq!(Solution::find_continuous_sequence(9),vec![vec![2,3,4],vec![4,5]]);
}