#![allow(unused)]
use std::{vec, hash};
use std::collections::HashMap;
use std::fmt;
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

 
// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

fn main() {
    struct Solution {
    }

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            // diff value : index
            let mut hash_diff_map: HashMap<i32, i32> = HashMap::new();
            // 1. Iterate over vector nums
            // 2. calculate diff (Target - num) num being current iteration of vec value
            // 3. Check if current num is present as a diff in the hashmap
            // 4. if diff exists, returned matched diff index against current num and its index
            
            for (idx, num) in nums.iter().enumerate(){ // 1.
                let mut diff = target-num; // 2. only need diff to insert
                    if let Some(&i) = hash_diff_map.get(&num){ // checking current num against previous diff inserted
                        // otherwise returning the returned value and the current idx
                        return vec![i as i32, idx as i32];
                    } else {
                        hash_diff_map.insert(diff, idx as i32);
                    }
            }
            return vec![];
        }
    }

    let mut nums = vec![3, 3];
    let mut target = 6;
    println!("solution : {:?}", Solution::two_sum(nums, target));
    // for num in Solution::two_sum(nums, target){
        // println!("num in indices: {}", num);
    // }
    
}
