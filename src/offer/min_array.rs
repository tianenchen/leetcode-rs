struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        if numbers.is_empty(){
            return 0
        }

        for (i,v) in numbers.iter().enumerate(){

            if let Some(n) = numbers.get(i+1) {
                if n < v {
                    return *n
                }
            }
        }

        numbers[0]

    }
}


#[test]
fn check() {
    assert_eq!(Solution::min_array(vec![3,4,5,1,2]),1);
    assert_eq!(Solution::min_array(vec![2,2,2,0,1]),0);
}