#![allow(unused)]

use std::{vec, io::Sink};
struct Solution{}

impl Solution {
    // Standard library implementation
    // nums.dedup();
    // nums.len() as i32;
    // writing our own instead

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // iterate through array, take note of copies by pushing true for copies, false for unique
        // to repeated_to_remove Vector

        // if nums is empty, just return len 0
        if nums.is_empty(){
            return 0;
        }

        let mut repeated_to_remove: Vec<bool> = vec![];
        let mut i = 0;
        for num in nums.iter() {
            if (i+1 < nums.len()){
                if (nums[i] == nums[i+1]){
                    repeated_to_remove.push(true)
                } else {
                    repeated_to_remove.push(false);
                }
            }

            i+=1;
        }

        repeated_to_remove.push(false); // fill in for last unique element, otherwise size mismatch causes panic
        let mut remove_iter = repeated_to_remove.iter();
        nums.retain(|_| !*remove_iter.next().unwrap()); // only retaining unique/elements that are false
        return nums.len() as i32;
    }
}

fn main() {
    let mut some_vec1 = Vec::from(vec![1, 2, 2, 2, 3, 3, 3]);
    let mut some_vec2 = Vec::from(vec![]);
    Solution::remove_duplicates(&mut some_vec1);
    Solution::remove_duplicates(&mut some_vec2);
    // assert_eq!()
}
