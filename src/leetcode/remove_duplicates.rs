struct Solution;

impl Solution {

    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut times = std::collections::LinkedList::new();
        let mut stack = std::collections::LinkedList::new();
        for c in s.chars(){
            if stack.is_empty() || *stack.back().unwrap()!=c{
                stack.push_back(c);
                times.push_back(1);
            }
            else{
                stack.push_back(c);
                *times.back_mut().unwrap()+=1;
                if *times.back().unwrap() == k{
                    for _ in 0..k{
                        stack.pop_back();
                    }
                    times.pop_back();
                }
            }
        }
        stack.into_iter().collect::<String>()
    }

    pub fn remove_duplicates1(s: String, k: i32) -> String {
        let k = k as usize;
        let mut stack = std::collections::LinkedList::new();
        for c in s.chars(){
            stack.push_back(c);
            let mut t = std::collections::LinkedList::new();
            if stack.len() < k{
                continue
            }
            for _ in 0..k{
                if let Some(back) = stack.back() {
                    if let Some(front) = t.front() {
                        if front!=back{
                            break
                        }
                    }
                    t.push_front(stack.pop_back().unwrap());
                }
                else{
                    break
                }
            }
            if t.len()!=k{
                stack.append(&mut t);
            }
        }
        stack.into_iter().collect::<String>()
    }
}

#[test]
fn check() {
    assert_eq!(Solution::remove_duplicates("abcd".to_string(),2),"abcd".to_string());
    assert_eq!(Solution::remove_duplicates("deeedbbcccbdaa".to_string(),3),"aa".to_string());
    assert_eq!(Solution::remove_duplicates("pbbcggttciiippooaais".to_string(),2),"ps".to_string());
}