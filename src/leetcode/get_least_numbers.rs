struct Solution;
impl Solution {
    pub fn get_least_numbers(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        Self::quick_sort(0,arr.len()-1,&mut arr,k);
        arr[0..k as usize].to_vec()
    }

    fn quick_sort(start :usize ,end :usize ,arr: &mut Vec<i32>, k: i32){
        if start < end{
            let base = arr[start];
            let (mut i,mut j) = (start,end);
            while i < j{
                while i < j && arr[j] >= base{//从后往前找，找到比base小的就交换
                    j-=1;
                }
                arr[i] = arr[j];
                while i < j && arr[i] < base{//从前往后找，找到比base大的就交换
                    i+=1;
                }
                arr[j] = arr[i];
            }
            arr[i] = base;
            if i == k as usize {//左边都小于arr[i]
                return;
            } 
            Self::quick_sort(i+1, end, arr, k);
            if i > 1{
                Self::quick_sort(start,i-1, arr, k);
            }
        }
    }
}


#[test]
fn check() {
    let mut v = vec![1,2,4,7,3];
    Solution::quick_sort(0,v.len()-1,&mut v,std::i32::MAX);
    assert_eq!(v,vec![1,2,3,4,7]);
}