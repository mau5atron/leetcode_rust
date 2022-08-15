// 15. 3Sum

// Given an integer array nums, return all the triplets 
// [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, 
// and nums[i] + nums[j] + nums[k] == 0.

// Notice that the solution set must not contain duplicate triplets.

 

// Example 1:

// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]
// Explanation: 
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
// The distinct triplets are [-1,0,1] and [-1,-1,2].
// Notice that the order of the output and the order of the triplets does not matter.

// Example 2:
// Input: nums = [0,1,1]
// Output: []
// Explanation: The only possible triplet does not sum up to 0.

// Example 3:
// Input: nums = [0,0,0]
// Output: [[0,0,0]]
// Explanation: The only possible triplet sums up to 0.
 

// Constraints:

// 3 <= nums.length <= 3000
// -105 <= nums[i] <= 105

struct Sol {}

impl Sol {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        return vec![vec![1], vec![2]];
    }
}

fn main() {
    println!("Hello, world!");
}

// How to solve notes:

/*

*/ 

#[cfg(test)]
mod tests {
    #[test]
    pub fn test1(){
        let test_vec: Vec<i32> = Vec::from(vec![-1,0,1,2,-1,-4]);
        let result: Vec<Vec<i32>> = Vec::from(vec![vec![-1,-1,2], vec![-1,0,1]]);
    }

    #[test]
    pub fn test2(){
        let test_vec: Vec<i32> = Vec::from(vec![0,1,1]);
        let result: Vec<i32> = Vec::from(vec![]);
    }

    #[test]
    pub fn test3(){
        let test_vec: Vec<i32> = Vec::from(vec![0,0,0]);
        let result: Vec<Vec<i32>> = Vec::from(vec![vec![0,0,0]]);
    }
}