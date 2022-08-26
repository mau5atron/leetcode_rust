#![allow(unused)]
// 9. Palindrome Number

// Given an integer x, return true if x is palindrome integer.

// An integer is a palindrome when it reads the same backward as forward.

// For example, 121 is a palindrome while 123 is not.
 

// Example 1:

// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
// Example 2:

// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
// Example 3:

// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 

// Constraints:

// -231 <= x <= 231 - 1


fn main() {
    struct Solution{
    };

    impl Solution {
        pub fn is_palindrome(x: i32) -> bool {
            // 1. convert integer to string
            // 2. reverse string
            // 3. compare original and reversed
            let mut init_str = x.clone().to_string();
            let mut rev_str: String = init_str.chars().rev().collect();
            
            return rev_str == init_str;
        }
    }

    Solution::is_palindrome(-512);
}
