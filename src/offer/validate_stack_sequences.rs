pub struct Solution;
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, mut popped: Vec<i32>) -> bool {
        let mut dummy = vec![];
        let (len, mut idx) = (pushed.len(), 0);
        while !popped.is_empty() {
            if idx == len {
                //走完了开始遍历栈
                if !dummy.is_empty() && *dummy.last().unwrap() == popped[0] {
                    dummy.remove(dummy.len() - 1);
                    popped.remove(0);
                    continue;
                } else {
                    break;
                }
            }
            if pushed[idx] == popped[0] {
                //遇到出队元素 模拟出队
                popped.remove(0);
                idx += 1;
            } else if !dummy.is_empty() && *dummy.last().unwrap() == popped[0] {
                //遇到出队元素 模拟出队
                dummy.remove(dummy.len() - 1);
                popped.remove(0);
            } else {
                dummy.push(pushed[idx]);
                idx += 1;
            }
        }
        return popped.is_empty();
    }
}

#[test]
fn check() {
    assert_eq!(
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]),
        true
    );
    assert_eq!(
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
        true
    );
    assert_eq!(
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]),
        true
    );
    assert_eq!(
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 1, 2]),
        false
    );
    assert_eq!(
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 2, 1]),
        true
    );
}
//pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
