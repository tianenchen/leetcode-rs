pub struct Solution;
impl Solution {
    pub fn get_least_numbers(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        // Self::quick_sort(0,arr.len()-1,&mut arr);
        // arr[0..k as usize].to_vec()
        Self::quick_sort2(k as usize,0,arr.len()-1,&mut arr);
        arr[0..k as usize].to_vec()
        // arr.sort();
        // arr[0..k as usize].to_vec()
    }

    fn quick_sort2(k :usize,left:usize,right:usize,arr: &mut Vec<i32>){
        if left<right{
            let base = arr[0];
            let (mut i,mut j) = (left,right);
            while i!=j{
                while i<j && arr[j]>= base{
                    j-=1;
                }
                arr[i] = arr[j];
                while i<j && arr[i]< base{
                    i+=1;
                }
                arr[j] = arr[i];
            }
            arr[i] = base;
            if i == k {
                return;
            }
            if i > 0 {
                Self::quick_sort2(k,left,i-1, arr);
            }
            Self::quick_sort2(k,i+1,right, arr);
        }
    }

    fn quick_sort(left:usize,right:usize,arr: &mut Vec<i32>){
        if left<right{
            let base = arr[left];
            let (mut i,mut j) = (left,right);
            while i!=j{
                while i<j && arr[j]>= base{
                    j-=1;
                }
                arr[i] = arr[j];
                while i<j && arr[i]< base{
                    i+=1;
                }
                arr[j] = arr[i];
            }
            arr[i] = base;
            if i > 0 {
                Self::quick_sort(left,i-1, arr);
            }
            Self::quick_sort(i+1,right, arr);
        }
    }
}