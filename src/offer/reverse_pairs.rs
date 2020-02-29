struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        // let len = nums.len();
        // let mut res = 0;
        // for i in 0..len{
        //     for j in i+1..len{
        //         if nums[i] > nums[j]{
        //             res+=1;
        //         }
        //     }
        // }
        // res
        if nums.is_empty() {return 0}
        let mut res = 0 ;
        let right = nums.len() - 1;
        Self::merge_sort(&mut nums,0,right,&mut res);
        res
    }

    fn merge_sort(nums :&mut Vec<i32>, left :usize , right :usize,res :&mut i32){
        if left<right{
            let mid = (left+right) >>1 ;
            Self::merge_sort(nums,left, mid,res);
            Self::merge_sort(nums,mid+1,right,res);
            Self::merge(nums,left,mid,right,res);
        }
    }

    fn merge(nums :&mut Vec<i32>,mut left :usize,mid :usize ,rigiht :usize,res :&mut i32){
        let mut tmp = vec![0;rigiht-left+1];
        let (mut i,mut j,mut idx) = (left,mid+1,0);
        while i <= mid && j <= rigiht{
            if nums[i] > nums[j] {
                *res += (mid-i+1) as i32;
                tmp[idx] = nums[j];
                j+=1;
            }
            else{
                tmp[idx] = nums[i];
                i+=1;
            }
            idx+=1;
        }
        while i<= mid{
            tmp[idx] = nums[i];
            i+=1;
            idx+=1;
        }
        while j<= rigiht{
            tmp[idx] = nums[j];
            j+=1;
            idx+=1;
        }
        idx = 0;
        while left <= rigiht{
            nums[left] = tmp[idx];
            left += 1;
            idx +=1;
        }
    }
}

#[test]
fn check() {
    assert_eq!(Solution::reverse_pairs(vec![7,5,6,4]),5);
}