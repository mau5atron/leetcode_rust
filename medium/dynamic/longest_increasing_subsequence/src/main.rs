// Given an integer array nums, return the length of the longest
// strictly increasing subsequence.

// A subsequence is a sequence that can be derived from an array by 
// deleting some or no elements without changing the order of the 
// remaining elements. For example, [3,6,2,7] is a subsequence of the 
// array [0,3,1,6,2,2,7].

 

// Example 1:
// Input: nums = [10,9,2,5,3,7,101,18]
// Output: 4
// Explanation: The longest increasing subsequence is 
// [2,3,7,101], therefore the length is 4.


// Example 2:
// Input: nums = [0,1,0,3,2,3]
// Output: 4



// Example 3:
// Input: nums = [7,7,7,7,7,7,7]
// Output: 1
 

// Constraints:

// 1 <= nums.length <= 2500
// -104 <= nums[i] <= 104

struct Solution {}

impl Solution {

    // O(n^2) runtime
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // fill vector of size nums with 1
        // long_incr_sub = longest_increasing_subsequence
        let mut long_incr_sub: Vec<i32> = Vec::from(vec![1; nums.len()]);
        let mut max: i32 = 1;
        let mut i: usize = 1;
        while i < nums.len() {
            let mut j: usize = 0;
            while j < i {
                if nums[i] > nums[j]{
                    long_incr_sub[i] = std::cmp::max(long_incr_sub[i], 1+long_incr_sub[j]);
                    // set the max value to the current longest increasing subsequence at index
                    max = std::cmp::max(max, long_incr_sub[i]);
                }
                j+=1;
            }

            i+=1;
        }
        
        return max;
    }

    // O(nlogn) runtime (much better)
    pub fn length_of_lis_2(nums: Vec<i32>) -> i32 {
        let mut subsequence: Vec<i32> = vec![];
        subsequence.push(nums[0]);        

        let mut i: usize = 0;
        while i < nums.len() {
            let last: i32 = subsequence[subsequence.len()-1];
            if nums[i] > last {
                subsequence.push(nums[i]);
            } else {
                let j = self::Solution::binary_search(&subsequence, nums[i]) as usize;
                subsequence[j] = nums[i];
            }

            i+=1;
        }

        return subsequence.len() as i32;
    }

    fn binary_search(subseq: &Vec<i32>, num: i32) -> i32 {
        let mut left: i32 = 0;
        // let right = sub.len()-1;
        let mut right: i32 = ((*subseq).len()-1) as i32;

        while left < right {
            let mid = (left + right)/2;

            if (subseq[mid as usize] < num){
                left = mid+1;
            } else {
                right = mid;
            }
        }
        
        return left;
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn test1(){
        let expected_result: i32 = 4;
        let nums = vec![10,9,2,5,3,7,101,18];
        assert_eq!(Solution::length_of_lis(nums), expected_result);
    }

    #[test]
    pub fn test2(){
        let expected_result: i32 = 4;
        let nums = vec![0,1,0,3,2,3];
        assert_eq!(Solution::length_of_lis(nums), expected_result);
    }

    #[test]
    pub fn test3(){
        let expected_result: i32 = 1;
        let nums = vec![7,7,7,7,7,7,7];
        assert_eq!(Solution::length_of_lis(nums), expected_result);
    }
}