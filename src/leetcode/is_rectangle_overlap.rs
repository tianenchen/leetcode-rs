pub struct Solution;

impl Solution {

    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        return !(rec1[2] <= rec2[0] || rec1[3] <= rec2[1] || rec1[0] >= rec2[2] || rec1[1] >= rec2[3])
    }

    //咋就没想出来呢
    pub fn is_rectangle_overlap_trash(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        if  rec1 == rec2 || (rec1[0]<rec2[0] && rec1[1]<rec2[1] && rec1[2] > rec2[2] && rec1[3] > rec2[3]) || (rec1[0]>rec2[0] && rec1[1]>rec2[1] && rec1[2] < rec2[2] && rec1[3] < rec2[3]){
            return true
        }
        let refer = vec![(rec1[0],rec1[1]),(rec1[2],rec1[3])];
        let points = vec![(rec2[0],rec2[1]),(rec2[2],rec2[1]),(rec2[0],rec2[3]),(rec2[2],rec2[3])];
        for point in &points{
            if point.0 > refer[0].0 && point.0 < refer[1].0 && point.1 > refer[0].1 && point.1 < refer[1].1 {
                return true
            }
        }
        let xs = vec![points[0].0,points[1].0];
        let ys = vec![points[0].1,points[1].1];
        for y in ys{
            for x in points[0].0..=points[3].0{
                if x > refer[0].0 && x < refer[1].0 && y > refer[0].1 && y < refer[1].1 {
                    return true
                }
            }
        }
        for x in xs{
            for y in points[0].1..=points[3].1{
                if x > refer[0].0 && x < refer[1].0 && y > refer[0].1 && y < refer[1].1 {
                    return true
                }
            }
        } 
        false
    }
}

#[test]
fn check() {
    assert_eq!(Solution::is_rectangle_overlap(vec![0,0,2,2], vec![1,1,3,3]),true);
    assert_eq!(Solution::is_rectangle_overlap(vec![0,0,1,1], vec![1,0,2,1]),false);
    assert_eq!(Solution::is_rectangle_overlap(vec![7,8,13,15], vec![10,8,12,20]),true);
    assert_eq!(Solution::is_rectangle_overlap(vec![5,15,8,18], vec![0,3,7,9]),false);
    assert_eq!(Solution::is_rectangle_overlap(vec![229,-132,833,333], vec![-244,-577,837,804]),true);
}