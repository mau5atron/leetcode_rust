// Given a 1-indexed array of integers numbers that is already sorted in 
// non-decreasing order, find two numbers such that they add up to a specific 
// target number. Let these two numbers be numbers[index1] and numbers[index2] 
// where 1 <= index1 < index2 <= numbers.length.

// Return the indices of the two numbers, index1 and index2, added by one as an 
// integer array [index1, index2] of length 2.

// The tests are generated such that there is exactly one solution. 
// You may not use the same element twice.

// Your solution must use only constant extra space.


// Example 1:

// Input: numbers = [2,7,11,15], target = 9
// Output: [1,2]
// Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].


// Example 2:

// Input: numbers = [2,3,4], target = 6
// Output: [1,3]
// Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].


// Example 3:

// Input: numbers = [-1,0], target = -1
// Output: [1,2]
// Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
 

// Constraints:

// 2 <= numbers.length <= 3 * 104
// -1000 <= numbers[i] <= 1000
// numbers is sorted in non-decreasing order.
// -1000 <= target <= 1000
// The tests are generated such that there is exactly one solution.

struct Sol {}

impl Sol {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: usize = 0; // init left side to 0 index
        let mut right = numbers.len() - 1; // init right side to last num at index

        while left < right {
            if numbers[left] + numbers[right] > target {
                // if left and right side of vector are added and are greater
                // than target, shift right side of vector to left
                right = right - 1;
            } else if numbers[left] + numbers[right] < target {
                // if left and right side of vector are added and are less than
                // target, shift left side of vector to the right
                left = left + 1;
            } else {
                // if top conditions are not met, then target has been found and
                // we should return a vector of the indexes that add up to the target
                return Vec::from(vec![(left+1) as i32, (right+1) as i32]);
            }
        }

        // return empty vector if above is not satisfied since we need to return a vector`
        // for the function
        return vec![];
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test1(){
        let test_vec = Vec::from(vec![2,7,11,15]);
        let target: i32 = 9;
        let expected_result: Vec<i32> = Vec::from(vec![1, 2]);

        assert_eq!(super::Sol::two_sum(test_vec, target), expected_result);
    }

    #[test]
    pub fn test2(){
        let test_vec = Vec::from(vec![2,3,4]);
        let target: i32 = 6;
        let expected_result: Vec<i32> = Vec::from(vec![1, 3]);
        assert_eq!(super::Sol::two_sum(test_vec, target), expected_result);
    }

    #[test]
    pub fn test3(){
        let test_vec = Vec::from(vec![-1,0]);
        let target: i32 = -1;
        let expected_result: Vec<i32> = Vec::from(vec![1, 2]);
        assert_eq!(super::Sol::two_sum(test_vec, target), expected_result);
    }
}