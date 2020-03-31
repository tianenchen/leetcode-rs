struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn cutting_rope(mut n: i32) -> i32 {
        // if n == 2{
        //     return 1
        // }
        // let mut res = 0;
        // for i in 1..n{
        //     res = std::cmp::max(res,std::cmp::max(i*(n-i),Self::cutting_rope(n-i)*i))
        // }
        // res

        // if n < 4 {return n-1}
        // let (s,p) = (n%3,n as u32/3);
        // if s == 0 {3_i32.pow(p)}
        // else if s == 1 {3_i32.pow(p-1)*4}
        // else{3_i32.pow(p)*2}

        if n < 4 {
            return n - 1;
        }
        let mut res = 1_u64;
        while n > 4 {
            res = res * 3 % 1000000007;
            n -= 3;
        }
        (res * n as u64 % 1000000007) as i32
    }
}

#[test]
fn check() {
    assert_eq!(Solution::cutting_rope(4), 4);
    assert_eq!(Solution::cutting_rope(10), 36);
    assert_eq!(Solution::cutting_rope(100), 703522804);
}

// f(n) = f(n-1)*1

/*

x =>  1 -> 1   2 -> 1  3-> 2


*/
