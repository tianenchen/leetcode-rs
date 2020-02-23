pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (len1,len2) = (nums1.len(),nums2.len());
        if nums1.is_empty(){
            if len2%2==0{
                return (nums2[len2/2]+nums2[len2/2-1]) as f64/2.0;
            }
            else{
                return nums2[len2/2] as f64;
            }
        }
        else if nums2.is_empty(){
            if len1%2==0{
                return (nums1[len1/2]+nums1[len1/2-1]) as f64/2.0;
            }
            else{
                return nums1[len1/2] as f64;
            }
        }
        let (mut i1,mut i2) = (0,0);
        let target = (len1+len2)/2;
        if (len1+len2) %2 == 0{
            let (mut res1,mut res2) = (0,0);
            while i1<len1||i2<len2{
                // println!("i1 :{} , i2 :{} , target :{}", i1,i2,target );
                if i1 == len1||(i2<len2&&nums1[i1]>=nums2[i2]){
                    if i1 + i2== target -1 {
                        res1 = nums2[i2];
                    }
                    else if i1 + i2== target{
                        res2 = nums2[i2];
                        break;
                    }
                    i2+=1;
                }
                else if i2==len2||(i1<len1&&nums2[i2]>=nums1[i1]){
                    if i1 + i2== target -1 {
                        res1 = nums1[i1];
                    }
                    else if i1 + i2== target{
                        res2 = nums1[i1];
                        break;
                    }
                    i1+=1;
                }
                // println!("res 1 :{} , res 2 :{}", res1,res2 );
            }
            // println!("res 1 :{} , res 2 :{}", res1,res2 );
            (res1+res2) as f64 /2.0
        }
        else {
            while i1<len1||i2<len2{
                // println!("i1 :{} , i2 :{} , target :{}", i1,i2,target );
                if i1 == len1||(i2<len2&&nums1[i1]>=nums2[i2]){
                    if i1 + i2 == target{
                        return nums2[i2] as f64;
                    }
                    i2+=1;
                }
                else{
                    if i1 + i2 == target{
                        return nums1[i1] as f64;
                    }
                    i1+=1;
                }
            }
            0.0
        }
    }
}

#[test]
fn check() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![0,0],vec![0,0]),0.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1,3],vec![2]),2.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![],vec![1]),1.0);
}