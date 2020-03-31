struct Solution;

#[allow(dead_code)]
impl Solution {
    // pub fn fib(n: i32) -> i32 {
    //     match n{
    //         0=>0,
    //         1=>1,
    //         _=>Self::fib(n-1)+Self::fib(n-2),
    //     }
    // }
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let (mut a, mut b) = (0, 1);
        let mut t;
        for _ in 2..=n {
            t = a; //上次的最大项
            a = b; //存储上次的最大项
            b += t; //上次的结果加上次最大的一项

            if b > 1000000007 {
                b %= 1000000007;
            }
        }
        b
    }
}

#[test]
fn check() {
    assert_eq!(Solution::fib(100), 687995182);
}

/*



3 =>    f(2) + f(1)    =>   f(1) + f(0) + f(1)  2


4 =>    f(3) + f(2)    =>   f(2) + f(1) + f(1) + f(0)  => f(1) + f(0) + f(1) + f(0) +f(1)   3


5 =>    f(4) + f(3)    =>   f(3) + f(1) + f(0) + f(1)  =>

*/
