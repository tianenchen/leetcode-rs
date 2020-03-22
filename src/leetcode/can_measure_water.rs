struct Solution;
impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut stack = std::collections::LinkedList::new();
        stack.push_back((0,0));
        while let Some((remain_x , remain_y)) = stack.pop_back() {
            if remain_x == z || remain_y == z || remain_x+remain_y == z {
                return true
            }
            else if visited.contains(&(remain_x,remain_y)){
                continue;
            }
            visited.insert((remain_x,remain_y));
            stack.push_back((0,remain_y));
            stack.push_back((x,remain_y));
            stack.push_back((remain_x,y));
            stack.push_back((remain_x-std::cmp::min(remain_x,y-remain_y),remain_y+std::cmp::min(remain_x,y-remain_y)));
            stack.push_back((remain_x+std::cmp::min(remain_y,x-remain_x),remain_y-std::cmp::min(remain_y,x-remain_x)));
        }
        false
    }
}


#[test]
fn check() {
    assert_eq!(Solution::can_measure_water(3, 5, 3),true);
    assert_eq!(Solution::can_measure_water(2, 6, 5),false);
    assert_eq!(Solution::can_measure_water(3, 5, 4),true);
}