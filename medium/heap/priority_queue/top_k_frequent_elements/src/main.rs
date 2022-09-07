// LeetCode # 347: Top K Frequent Elements


// Given an integer array nums and an integer k, 
// return the k most frequent elements. You may return the answer in any order.


// Example 1:
// Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]



// Example 2:
// Input: nums = [1], k = 1
// Output: [1]
 

// Constraints:

// 1 <= nums.length <= 105
// -104 <= nums[i] <= 104
// k is in the range [1, the number of unique elements in the array].
// It is guaranteed that the answer is unique.

use std::collections::HashMap;


struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num_frequency = HashMap::new();

        for &num in nums.iter(){
            *num_frequency.entry(num).or_insert(0) += 1;
        }

        let mut sorted = vec![Vec::new(); nums.len()];

        for (&num, &num_count) in num_frequency.iter(){
            sorted[nums.len() - num_count].push(num);
        }

        return sorted.into_iter().flatten().take(k as usize).collect();
    }
}

#[cfg(test)]
mod tests {
    pub fn test1(){

    }
}

fn main() {
    
}
