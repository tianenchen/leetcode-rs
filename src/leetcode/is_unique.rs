/*
实现一个算法，确定一个字符串 s 的所有字符是否全都不同。

示例 1：

输入: s = "leetcode"
输出: false 
示例 2：

输入: s = "abc"
输出: true
限制：

0 <= len(s) <= 100
如果你不使用额外的数据结构，会很加分。
*/


struct Solution;
impl Solution {
    pub fn is_unique(astr: String) -> bool {
        let mut v = vec!['/';100];
        for c in astr.chars(){
            let mut idx = c as usize%100;
            if v[idx] == '/'{
                v[idx] = c;
            }
            else {
                while v[idx]=='/' || v[idx]!=c{
                    idx+=1;
                    if idx==100{
                        idx = 0;
                    }
                }
                if v[idx]==c{
                    return false
                }
                else{
                    v[idx] = c;
                }
            }
        }
        true
    }
}