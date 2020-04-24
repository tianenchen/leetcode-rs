struct Solution;

#[derive(PartialEq)]
enum State {
    Origin,
    Rising,
    Downing,
}

impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let len = a.len();
        if len < 3 {
            return false;
        }
        let mut state = State::Origin;
        for i in 1..len {
            if a[i] == a[i - 1] {
                return false;
            }
            if a[i] > a[i - 1] {
                match state {
                    State::Origin => state = State::Rising,
                    State::Downing => return false,
                    _ => (),
                }
            } else {
                //开始下沉
                match state {
                    State::Origin => return false,
                    State::Rising => state = State::Downing,
                    _ => (),
                }
            }
        }
        State::Downing == state
    }
}

#[test]
fn check() {
    assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
    assert_eq!(Solution::valid_mountain_array(vec![0, 0, 3, 2, 1]), false);
    assert_eq!(Solution::valid_mountain_array(vec![0, 1, 3, 2, 1]), true);
    assert_eq!(
        Solution::valid_mountain_array(vec![0, 1, 3, 2, 1, 2]),
        false
    );
    assert_eq!(Solution::valid_mountain_array(vec![0, 1, 3, 4]), false);
    assert_eq!(Solution::valid_mountain_array(vec![4, 3, 2, 1, 0]), false);
}
