// LeetCode #42 Trapping Rain Water

// Given n non-negative integers representing an elevation map where 
// the width of each bar is 1, compute how much water it can trap after raining.

// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
// Output: 6
// Explanation: The above elevation map (black section) is 
// represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 
// 6 units of rain water (blue section) are being trapped.


// Example 2:

// Input: height = [4,2,0,3,2,5]
// Output: 9

struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0; // left ptr
        let mut right = height.len() - 1; // right ptr

        let mut total_amount_of_water = 0; // total
        let mut left_max = height[0]; // left ptr height
        let mut right_max = height[right]; // right ptr height

        while left < right {
            if height[left] < height[right] {
                left_max = std::cmp::max(left_max, height[left]);
                if (left_max - height[left] > 0) {
                    total_amount_of_water = total_amount_of_water + (left_max - height[left]);
                }

                left += 1;
            } else {
                right_max = std::cmp::max(right_max, height[right]);
                if (right_max - height[right] > 0){
                    total_amount_of_water = total_amount_of_water + (right_max - height[right]);
                }

                right -= 1;
            }
        }

        return total_amount_of_water;
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn test1(){
        let expected_result: i32 = 6;
        let input: Vec<i32> = Vec::from(vec![0,1,0,2,1,0,1,3,2,1,2,1]);

        assert_eq!(Solution::trap(input), expected_result);
    }

    #[test]
    pub fn test2(){
        let expected_result: i32 = 9;
        let input: Vec<i32> = Vec::from(vec![4,2,0,3,2,5]);
        assert_eq!(Solution::trap(input), expected_result);
    }
}