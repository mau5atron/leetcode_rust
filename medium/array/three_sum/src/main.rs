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
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        // let mut result_list: Vec<Vec<i32>> = Vec::from(vec![vec![]]);
        // just init vec instead of adding a whole empty list
        let mut result_list: Vec<Vec<i32>> = Vec::new();
        let borrowed_list: &mut Vec<i32> = &mut nums;

        for (idx, _num) in borrowed_list.iter().enumerate(){
            if borrowed_list[idx] <= 0 && (idx == 0 || borrowed_list[idx] != borrowed_list[idx-1]){
                Self::two_sum_two(borrowed_list, idx, &mut result_list)
            }
        }

        return result_list;
    }
    
    fn two_sum_two(nums: &Vec<i32>, idx: usize, result_list: &mut Vec<Vec<i32>>){
        let mut left: usize = idx + 1;
        let mut right: usize = nums.len()-1;

        while left < right {
            let sum: i32 = nums[idx] + nums[left] + nums[right];
            
            if sum < 0 {
                left = left + 1;
            } else if sum > 0 {
                right = right - 1;
            } else {
                if sum == 0 {
                    result_list.push(Vec::from(vec![nums[idx], nums[left], nums[right]]));
                    left+=1; // increment to make sure we don't go out of bounds
                    // in some scenarios
                    while left < right && nums[left] == nums[left-1]{
                        left = left + 1;
                    }
                }
            }
        }
    }
}

fn main() {
    // tests::test1();
    // self::tests;
    // self::tests::test1();
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test1(){
        let test_vec: Vec<i32> = Vec::from(vec![-1,0,1,2,-1,-4]);
        let result: Vec<Vec<i32>> = Vec::from(vec![vec![-1,-1,2], vec![-1,0,1]]);
        assert_eq!(super::Sol::three_sum(test_vec), result);
    }

    #[test]
    pub fn test2(){
        let test_vec: Vec<i32> = Vec::from(vec![0,1,1]);
        let result: Vec<Vec<i32>> = Vec::new();
        assert_eq!(super::Sol::three_sum(test_vec), result);
    }

    #[test]
    pub fn test3(){
        let test_vec: Vec<i32> = Vec::from(vec![0,0,0]);
        let result: Vec<Vec<i32>> = Vec::from(vec![vec![0,0,0]]);
        assert_eq!(super::Sol::three_sum(test_vec), result);
    }
}