// Given an integer array nums, find a contiguous non-empty subarray within 
// the array that has the largest product, and return the product.

// The test cases are generated so that the answer will fit in a 32-bit integer.

// A subarray is a contiguous subsequence of the array.

 

// Example 1:
// Input: nums = [2,3,-2,4]
// Output: 6
// Explanation: [2,3] has the largest product 6.

// Example 2:
// Input: nums = [-2,0,-1]
// Output: 0
// Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
 
// Constraints:

// 1 <= nums.length <= 2 * 104
// -10 <= nums[i] <= 10
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

struct Solution {}

impl Solution {
    // pub fn max_product(nums: Vec<i32>) -> i32 {
    //     let mut current_product: i32 = 0;

    //     for (idx, num) in nums.iter().enumerate(){
    //         if num < &1{
    //             continue;
    //         }

    //         if (idx+1) < nums.len() && nums[idx+1].is_positive(){
    //             current_product = num*(nums[idx + 1]);
    //         }
    //         println!("current num: {}", num);
    //     }
    //     println!("Current product : {}", current_product);
    //     return current_product;
    // }


//     pub fn max_product(nums: Vec<i32>) -> i32 {
//         if nums.len() < 2 {
//             return nums[0];
//         }
        
//         let mut current_product: i32 = 0;

//         for (idx, num) in nums.iter().enumerate(){

//             if (idx+1) < nums.len(){
//                 let product = num*(nums[idx + 1]);
//                 // println!("product : {}", product);
//                 if product.is_positive() && current_product < 1{
//                     current_product = product;
//                 }
//             }
            
//             // if (idx+1) < nums.len() && nums[idx+1].is_negative(){
//             //     current_product = 
//             // }
//             // println!("current num: {}", num);
//         }
//         // println!("Current product : {}\n", current_product);
//         return current_product;
//     }
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut min_product = 1;
        let mut max_product = 1;
        let mut result = nums[0];

        for (idx, num) in nums.iter().enumerate(){
            let temp_max_product = max_product * nums[idx];
            max_product = std::cmp::max(nums[idx], std::cmp::max(nums[idx]*max_product, nums[idx]*min_product));
            min_product = std::cmp::min(nums[idx], std::cmp::min(temp_max_product, nums[idx]*min_product));

            result = std::cmp::max(result, std::cmp::max(max_product, min_product));
        }

        return result;
    }
}

fn main() {
    Solution::max_product(Vec::from(vec![-3, 3, 4, -2, 4]));
    Solution::max_product(Vec::from(vec![-3, -1, -1]));
}
