struct Solution;
impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let (mut i,len) = (0,a.len());
        let target = a.iter().sum::<i32>()/3;
        let mut sum0 = 0;
        while i<len-2{
            for idx in i..len-2{
                sum0 += a[idx];
                if sum0 == target{
                    i = idx;
                    break;
                }
            }
            if sum0!= target{
                return false
            }
            let mut sum1= 0;
            let mut j = i+1;
            for idx in j..len-1{
                sum1 += a[idx];
                if sum1 == target{
                    j = idx;
                    break;
                }
            }
            if sum0 == sum1&& sum0 == target && a[j+1..len].iter().sum::<i32>() == target{
                return true
            }
            else{
                i+=1;
            }
        }
        false
    }
    pub fn can_three_parts_equal_sum2(a: Vec<i32>) -> bool {
        let (mut i , mut j,len) = (0,2,a.len());
        while j<len{
            let (sum0,mut sum1,mut sum2) = (a[0..=i].iter().sum::<i32>(),a[i+1..j].iter().sum::<i32>(),a[j..len].iter().sum::<i32>());
            // dbg!(sum0,sum1,sum2);
            if sum0 == sum1 && sum0 == sum2{
                return true
            }
            for k in j..len-1{
                sum1+=a[k];
                sum2-=a[k];
                if sum0 == sum1 && sum0 == sum2{
                    return true
                }
            }
            i+=1;
            j=i+2;
        }
        false
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn check() {
        assert_eq!(Solution::can_three_parts_equal_sum(vec![2,1,1,2]),true);
        assert_eq!(Solution::can_three_parts_equal_sum(vec![1,1,1]),true);
        assert_eq!(Solution::can_three_parts_equal_sum(vec![1,1,2]),false);
        assert_eq!(Solution::can_three_parts_equal_sum(vec![1,4,5,4,1]),true);
        assert_eq!(Solution::can_three_parts_equal_sum(vec![1,-1,1,-1]),false);
        assert_eq!(Solution::can_three_parts_equal_sum(vec![6,1,1,13,-1,0,-10,20]),false);
    }
}