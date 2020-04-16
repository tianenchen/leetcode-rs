struct Solution;

impl Solution {
    pub fn intersection(start1: Vec<i32>, end1: Vec<i32>, start2: Vec<i32>, end2: Vec<i32>) -> Vec<f64> {
        let start1 = start1.into_iter().map(|x|x as f64).collect::<Vec<f64>>();
        let end1 = end1.into_iter().map(|x|x as f64).collect::<Vec<f64>>();
        let start2 = start2.into_iter().map(|x|x as f64).collect::<Vec<f64>>();
        let end2 = end2.into_iter().map(|x|x as f64).collect::<Vec<f64>>();
        let (x1,y1) = (start1[0],start1[1]);
        let (x2,y2) = (start2[0],start2[1]);
        let k1 = (end1[1]-start1[1])/(end1[0]-start1[0]);
        let k2 = (end2[1]-start2[1])/(end2[0]-start2[0]);
        let (x,y) ;
        //平行
        if (k1-k2).abs() < 1e-6{
            let rx1 = x1-y1/k1;
            let rx2 = x2-y2/k2;
            let ry1 = y1-k1*x1;
            let ry2 = y2-k2*y2;
            //与x轴交点或y轴交点相同
            if (rx1-rx2).abs() < 1e-6 || (ry1-ry2).abs() < 1e-6{
                //同一条直线
                //give up
            }
            x=1.0;
            y=1.0;
        }
        else{
            x = (k1*x1-k2*x2+y2-y1)/(k1-k2);
            y = (k2*y1-k1*y2+k1*k2*(x2-x1))/(k2-k1);
            //交点位于界外
            // if ((x > start1[0] && x > end1[0]) || (x < start1[0] && x < end1[0])) || (x < start1[0] && x < end1[0] && x < start2[0] && x < end2[0]){
            //     return vec![]
            // }
        }

        vec![x,y]
    }
}