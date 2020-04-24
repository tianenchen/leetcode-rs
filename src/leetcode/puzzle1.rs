//分数结构体
#[derive(Debug, PartialEq, Eq)]
struct Fraction {
    //分子
    pub numerator: i32,
    //分母
    pub denominator: i32,
}

impl Fraction {
    fn new(numerator: i32, denominator: i32) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }
}

struct Solution;

//求两分数的均值
impl Solution {
    fn get_two_fraction_average(a: Fraction, b: Fraction) -> Fraction {
        //排除分母为0
        assert_ne!(a.denominator, 0);
        assert_ne!(b.denominator, 0);

        //分子为0
        if a.numerator == 0 {
            return b;
        }
        if b.numerator == 0 {
            return a;
        }

        //求分母最小公倍数
        let common_multiple =
            (a.denominator * b.denominator) / Self::gcd(a.denominator, b.denominator);
        //求a,b分母乘数
        let mult_a = common_multiple / a.denominator;
        let mult_b = common_multiple / b.denominator;

        Fraction::new(mult_a * a.numerator + mult_b * b.numerator, common_multiple)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a % b == 0 {
            b
        } else {
            Self::gcd(b, a % b)
        }
    }
}

//下面是测试类
#[test]
fn check() {
    let a = Fraction::new(1, 5);
    let b = Fraction::new(2, 3);
    assert_eq!(
        Solution::get_two_fraction_average(a, b),
        Fraction::new(13, 15)
    );

    let a = Fraction::new(1, 3);
    let b = Fraction::new(5, 9);
    assert_eq!(
        Solution::get_two_fraction_average(a, b),
        Fraction::new(8, 9)
    );

    let a = Fraction::new(0, 5);
    let b = Fraction::new(2, 3);
    assert_eq!(
        Solution::get_two_fraction_average(a, b),
        Fraction::new(2, 3)
    );
}
