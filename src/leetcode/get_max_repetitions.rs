struct Solution;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let (n1, n2) = (n1 as usize, n2 as usize);
        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let (len1, len2) = (s1.len(), s2.len());
        let (mut i, mut j) = (0, 0);
        let mut flg = None;
        let mut res = 0;
        while i < len1 * n1 {
            //顺序遍历找到与s2中元素相同的位置
            if s1[i % len1] == s2[j % len2] {
                // dbg!(j % (len2 * n2));
                if j % (len2 * n2) == 0 {
                    //第一次找到
                    if let Some(flg) = flg {
                        // dbg!(flg, i, flg % len1, i % len1);
                        if flg % len2 == i % len2 {
                            //发现循环入口
                            //dbg!(i, flg, len1 * n1 - flg, i - flg);
                            return ((len1 * n1 - flg) / (i - flg)) as i32;
                        }
                    }
                    flg = Some(i);
                }
                if j % (len2 * n2) == len2 * n2 - 1 {
                    res += 1;
                }
                j += 1;
            }
            i += 1;
        }
        res
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::get_max_repetitions("acb".to_string(), 4, "ab".to_string(), 2),
        2
    );
    assert_eq!(
        Solution::get_max_repetitions("aaa".to_string(), 3, "aa".to_string(), 1),
        4
    );
    // assert_eq!(Solution::get_max_repetitions("bacaba".to_string(),3,"abacab".to_string(),1),2);
}
