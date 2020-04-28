struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        if num.is_empty() {
            return String::from("0");
        }
        let mut cs = num.chars().collect::<Vec<char>>();
        let mut count = 0;
        while count < k {
            let len = cs.len();
            for i in 0..len {
                if i == len - 1 || cs[i] > cs[i + 1] {
                    cs.remove(i);
                    count += 1;
                    break;
                }
            }
        }
        let res = cs
            .into_iter()
            .collect::<String>()
            .trim_start_matches('0')
            .to_string();
        if res.is_empty() {
            "0".to_owned()
        } else {
            res.to_owned()
        }
        // cs.into_iter()
        //     .fold(0, |l, r| {
        //         l * 10 + r.to_digit(10).or_else(|| Some(0)).unwrap()
        //     })
        //     .to_string()
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::remove_kdigits("10".to_string(), 2),
        "0".to_string()
    );
}
