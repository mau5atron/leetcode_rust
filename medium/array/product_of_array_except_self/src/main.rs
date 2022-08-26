// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time and without using the division operation.

 

// Example 1:

// Input: nums = [1,2,3,4]
// Output: [24,12,8,6]
// Example 2:

// Input: nums = [-1,1,0,-3,3]
// Output: [0,0,9,0,0]
 

// Constraints:

// 2 <= nums.length <= 105
// -30 <= nums[i] <= 30
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
 

// Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as extra space for space complexity analysis.)

struct Solution {}

impl Solution {
    // pub fn recursive_helper(nums: Vec<i32>, )

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // for (idx, current_num) in nums.iter().enumerate(){}
        // return self::Solution::product_except_self(nums)
   /*      for num in nums.iter(){
            return nums;
        }

        return nums; */


        let mut result: Vec<i32> = Vec::from(vec![1; nums.len()]);
        // fill up result vector with 1

        let mut prefix: i32 = 1;
        let mut postfix: i32 = 1;

        // left to right
        for (idx, num) in nums.iter().enumerate(){
            result[idx] = prefix;
            prefix = num*prefix;
        }

        // right to left
        for (idx, num) in nums.iter().enumerate().rev(){
            result[idx] = result[idx]*postfix;
            postfix = num*postfix;
        }

        return result; // solution is O(2n) or O(n) runtime, O(1) space
    }
}

fn main() {
    println!("SOL: {:?}", Solution::product_except_self(Vec::from(vec![1, 2, 3, 4])));
}
