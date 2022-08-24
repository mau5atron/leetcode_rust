// LeetCode 11: Container with Most Water

// You are given an integer array height of length n. There are n vertical 
// lines drawn such that the two endpoints of the ith line are (i, 0) and 
// (i, height[i]).

// Find two lines that together with the x-axis form a container, such that 
// the container contains the most water.

// Return the maximum amount of water a container can store.

// Notice that you may not slant the container.


// Example 1:
// Input: height = [1,8,6,2,5,4,8,3,7]
// Output: 49
// Explanation: The above vertical lines are represented by array 
// [1,8,6,2,5,4,8,3,7]. In this case, the max area of water 
// (blue section) the container can contain is 49.


// Example 2:
// Input: height = [1,1]
// Output: 1

// Constraints:

// n == height.length
// 2 <= n <= 105
// 0 <= height[i] <= 104


struct Sol {}

impl Sol {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            let width = std::cmp::min(height[left], height[right]);
            let length = right - left;

            max = std::cmp::max(max, width*length as i32);
            println!("max: {}", max);
            if height[left] < height[right]{
                left+=1;
            } else {
                right-=1;
            } 
        }
        
        return max;
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::Sol;

    #[test]
    pub fn test1(){
        let height: Vec<i32> = Vec::from(vec![1,8,6,2,5,4,8,3,7]);
        let expected_output: i32 = 49;
        assert_eq!(expected_output, Sol::max_area(height));
    }

    #[test]
    pub fn test2(){
        let height: Vec<i32> = Vec::from(vec![1, 1]);
        let expected_result: i32 = 1;
        assert_eq!(expected_result, Sol::max_area(height));
    }
}