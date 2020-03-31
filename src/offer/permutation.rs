struct Solution;
impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        let cs = s.chars().collect::<Vec<char>>();
        let mut res = std::collections::HashSet::new();
        Self::help(&cs, String::default(), &mut res);
        res.into_iter().collect::<Vec<String>>()
    }

    fn help(cs: &Vec<char>, s: String, res: &mut std::collections::HashSet<String>) {
        let len = cs.len();
        if len == 0 {
            res.insert(s);
        } else {
            for i in 0..len {
                // let mut next_cs = Vec::from(&cs[0..i]);
                // next_cs.append(&mut Vec::from(&cs[i+1..len]));
                let mut next_cs = cs.clone();
                let c = next_cs.remove(i);
                Self::help(&next_cs, s.clone() + &c.to_string(), res);
            }
        }
    }
}

/*

输入：s = "abc"
输出：["abc","acb","bac","bca","cab","cba"]

*/
