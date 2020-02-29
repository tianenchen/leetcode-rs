struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = std::collections::HashSet::new();
        let (mut head,mut tail) = (0,0);
        let cs = s.chars().collect::<Vec<char>>();
        let mut max = 0;
        while tail < cs.len(){
            if let Some(_) = set.get(&cs[tail]) {
                set.remove(&cs[head]);
                head+=1;
            }
            else{
                set.insert(&cs[tail]);
                tail+=1;
                if tail-head > max{ max = tail - head ;}
            }
        }
        max as i32
    }
}