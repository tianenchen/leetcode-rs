struct Solution;

impl Solution {
    pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for v in b{
            let (mut min,mut idx) = (std::i32::MAX,0);
            let (mut m,mut mi) = (a.first().unwrap(),0);
            let mut changed = false;
            for (i,t) in a.iter().enumerate(){
                if m > t {
                    m = t;
                    mi = i;
                }
                let tt = t- v;
                if tt > 0 && tt < min{
                    idx = i;
                    min = tt;
                    changed = true;
                }
            }
            if !changed{
                res.push(a.remove(mi))
            }
            else{
                res.push(a.remove(idx));
            }
        }
        res
    }
}
#[test]
fn check() {
    assert_eq!(Solution::advantage_count(vec![2,7,11,15], vec![1,10,4,11]),vec![2,11,7,15]);
    assert_eq!(Solution::advantage_count(vec![12,24,8,32], vec![13,25,32,11]),vec![24,32,8,12]);
}