#![allow(unused)]

pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        } else if nums.len() == 1 {
            return nums[0];
        }
        let mut l: i32 = 0;
        let mut mid: i32 = 0;
        let mut r: i32 =  (nums.len() - 1) as i32;

        while r > l {
          mid = l + (r - l) / 2;
          if nums[mid as usize] > nums[r as usize] {
            l = mid + 1;
          } else {
            r = mid;
          }
        }

        nums[l as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_153() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }
}
