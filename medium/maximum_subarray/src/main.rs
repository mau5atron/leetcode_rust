// Given an integer array nums, find the contiguous subarray 
// (containing at least one number) which has the largest sum and return its sum.

// A subarray is a contiguous part of an array.

 

// Example 1:

// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.

// Example 2:
// Input: nums = [1]
// Output: 1


// Example 3:
// Input: nums = [5,4,-1,7,8]
// Output: 23
 

// Constraints:

// 1 <= nums.length <= 105
// -104 <= nums[i] <= 104
 

// Follow up: If you have figured out the O(n) solution, 
// try coding another solution using the divide and conquer approach, which is more subtle.
use std::time::Instant;
struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut currentSum: i32 = 0;
        let mut maximumSum: i32 = nums[0];

        for num in nums.iter(){
            if currentSum < 0 {
                currentSum = 0;
            }

            currentSum = currentSum + num;
            maximumSum = std::cmp::max(currentSum, maximumSum);
        }

        return maximumSum;
    }
}

fn main() {
    let before = Instant::now();
    let sol1: i32 = Solution::max_sub_array(Vec::from(vec![-2,1,-3,4,-1,2,1,-5,4]));
    println!("Solution to [-2,1,-3,4,-1,2,1,-5,4] : {}, time to exec : {:?}", sol1, before.elapsed());
}
