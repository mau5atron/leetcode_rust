// LeetCode # 128: Longest Consecutive Sequence

// Given an unsorted array of integers nums, 
// return the length of the longest consecutive elements sequence.

// You must write an algorithm that runs in O(n) time.

 

// Example 1:

// Input: nums = [100,4,200,1,3,2]
// Output: 4
// Explanation: The longest consecutive elements sequence is 
// [1, 2, 3, 4]. Therefore its length is 4.

// Example 2:
// Input: nums = [0,3,7,2,5,8,4,6,0,1]
// Output: 9
 

// Constraints:

// 0 <= nums.length <= 105
// -109 <= nums[i] <= 109

use std::collections::HashSet;

struct Sol;

impl Sol {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // check if list len is empty
        if nums.len() == 0{
            return 0;
        }

        let mut nums_hash_set: HashSet<i32> = HashSet::new();

        for num in nums.iter(){
            nums_hash_set.insert(*num);
        }

        let mut longest_seq: i32 = 1;

        for num in nums_hash_set.iter(){
            if nums_hash_set.contains(&(num - 1)){
                continue;
            } else {
                let mut current_num = num.clone();
                let mut current_subseq = 1;

                while nums_hash_set.contains(&(current_num+1)){
                    current_num += 1;
                    current_subseq += 1;
                }

                longest_seq = std::cmp::max(longest_seq, current_subseq);
            }
        }

        return longest_seq;
    }
}

#[cfg(test)]
mod tests {
    use crate::Sol;

    #[test]
    pub fn test1(){
        let input = vec![100,4,200,1,3,2];
        let expected_result = 4;

        assert_eq!(Sol::longest_consecutive(input), expected_result);
    }

    #[test]
    pub fn test2(){
        let input = vec![0,3,7,2,5,8,4,6,0,1];
        let expected_result = 9;

        assert_eq!(Sol::longest_consecutive(input), expected_result);
    }
}

fn main() {
    println!("Hello, world!");
}
