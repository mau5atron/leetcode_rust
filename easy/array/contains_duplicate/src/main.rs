// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

 

// Example 1:

// Input: nums = [1,2,3,1]
// Output: true
// Example 2:

// Input: nums = [1,2,3,4]
// Output: false
// Example 3:

// Input: nums = [1,1,1,3,3,4,3,2,4,2]
// Output: true
 

// Constraints:

// 1 <= nums.length <= 105
// -109 <= nums[i] <= 109

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut present_num_hash: HashMap<i32, i32> = HashMap::new();
        let mut idx = 0;

        for num in nums.iter(){
            if present_num_hash.contains_key(num){
                return true;
            }

            present_num_hash.insert(*num, idx);
            idx += 1;
        }

        return false;
    }
}

// look up dereferncing in rust

fn main() {
    let prob1 = Solution::contains_duplicate(Vec::from(vec![1, 3, 2, 1]));
    let prob2 = Solution::contains_duplicate(Vec::from(vec![1, 2, 3, 4]));
    println!("prob1 = {}", prob1);
    println!("prob2 = {}", prob2);
    let prob3: bool = Solution::contains_duplicate(Vec::from(vec![1,1,1,3,3,4,3,2,4,2]));
    println!("prob3 = {}", prob3);
}
