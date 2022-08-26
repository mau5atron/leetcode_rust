// 153. Find Minimum in Rotated Sorted Array

// Suppose an array of length n sorted in ascending order is rotated 
// between 1 and n times. For example, the array nums = [0,1,2,4,5,6,7] might become:

// [4,5,6,7,0,1,2] if it was rotated 4 times.
// [0,1,2,4,5,6,7] if it was rotated 7 times.
// Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].

// Given the sorted rotated array nums of unique elements, return the minimum element of this array.

// You must write an algorithm that runs in O(log n) time.


// Example 1:
// Input: nums = [3,4,5,1,2]
// Output: 1
// Explanation: The original array was [1,2,3,4,5] rotated 3 times.

// Example 2:
// Input: nums = [4,5,6,7,0,1,2]
// Output: 0
// Explanation: The original array was [0,1,2,4,5,6,7] and it was rotated 4 times.

// Example 3:
// Input: nums = [11,13,15,17]
// Output: 11
// Explanation: The original array was [11,13,15,17] and it was rotated 4 times. 
 

// Constraints:

// n == nums.length
// 1 <= n <= 5000
// -5000 <= nums[i] <= 5000
// All the integers of nums are unique.
// nums is sorted and rotated between 1 and n times.

struct Sol {} // solution struct

impl Sol {
    // seems like something related to binary search
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left_side_index = 0;
        let mut right_side_index = nums.len() - 1;
        let mut minimum_value = nums[0];

        while left_side_index <= right_side_index {
            // left side, index at 0 < than right_side_index index on final element
            if nums[left_side_index] < nums[right_side_index]{
                // compare current minimum value against element at 0 index
                // and assign it to the current minimum value
                minimum_value = std::cmp::min(minimum_value, nums[left_side_index]);
            }

            // determine middle of the vector by adding both left and right side indexes
            // indicating first and final elements in vector, then dividing by 2
            let mut middle = (left_side_index + right_side_index) / 2; // will give regular integer of usize, not floating point

            // set minimum value again by comparing current minimum value with the element
            // at the midpoint
            minimum_value = std::cmp::min(minimum_value, nums[middle]);

            // if the first element (left side) is less or equal to element at the midpoint
            // reassign left side with the midpoint index + 1
            if nums[left_side_index] <= nums[middle]{
                left_side_index = middle + 1;
            } else {
            // else reassign right index with middle index - 1
                right_side_index = middle - 1;
            }
        }

        return minimum_value;
    }
}

fn main() {
    
}
