struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for i in &matrix[..]{
            if i.is_empty(){
                return false;
            }
            else if i[0] == target{
                return true;
            }
            else if i[0] > target {
                continue
            }
            for l in &i[..]{
                if l == &target{
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn check() {
    assert_eq!(Solution::find_number_in2_d_array(vec![vec![6,7,8,9],vec![1,2,3,4]],9),true);
    assert_eq!(Solution::find_number_in2_d_array(vec![vec![6,7,8,9],vec![1,2,3,4]],10),false);
}