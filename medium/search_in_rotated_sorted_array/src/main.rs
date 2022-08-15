// 33. Search in Rotated Sorted Array

// There is an integer array nums sorted in ascending order (with distinct values).

// Prior to being passed to your function, nums is possibly rotated at an unknown 
// pivot index k (1 <= k < nums.length) such that the resulting array is 
// [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). 
// For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].

// Given the array nums after the possible rotation and an integer target, return 
// the index of target if it is in nums, or -1 if it is not in nums.

// You must write an algorithm with O(log n) runtime complexity.

// Example 1:
// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4

// Example 2:
// Input: nums = [4,5,6,7,0,1,2], target = 3
// Output: -1

// Example 3:
// Input: nums = [1], target = 0
// Output: -1
 

// Constraints:

// 1 <= nums.length <= 5000
// -104 <= nums[i] <= 104
// All values of nums are unique.
// nums is an ascending array that is possibly rotated.
// -104 <= target <= 104

struct Sol {}

impl Sol {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // init left and right side of array index
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mut mid = (left + right) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            // check if left section is sorted
            if nums[left] <= nums[mid]{
                // if this is true, means numbers from left to mid are already
                // in ascending order
                if (target < nums[left]) || (target > nums[mid]){
                    // move left pointer to mid + 1
                    left = mid + 1;
                } else {
                    // move right pointer from right to mid - 1
                    right = mid - 1;
                }
            } else {
                if (target < nums[mid]) || (target > nums[right]){
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }

        return -1; // if num does not exist, return -1
    }
}

fn main() {
}


#[cfg(test)]
mod tests {
    #[test]
    fn test1(){
        let result: i32 = 4;
        assert_eq!(result, super::Sol::search(Vec::from(vec![4,5,6,7,0,1,2]), 0));
    }

    #[test]
    fn test2(){
        let result = -1;
        assert_eq!(result, super::Sol::search(Vec::from(vec![4,5,6,7,0,1,2]), 3))
    }

    #[test]
    fn test3(){
        let result = -1;
        assert_eq!(result, super::Sol::search(Vec::from(vec![1]), 0));
    }
}
