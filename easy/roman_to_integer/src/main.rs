#![allow(unused)]
// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
// 
// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
// 
// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
// 
// I can be placed before V (5) and X (10) to make 4 and 9. 
// X can be placed before L (50) and C (100) to make 40 and 90. 
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given a roman numeral, convert it to an integer.
// 
//  
// 
// Example 1:
// 
// Input: s = "III"
// Output: 3
// Explanation: III = 3.
// Example 2:
// 
// Input: s = "LVIII"
// Output: 58
// Explanation: L = 50, V= 5, III = 3.
// Example 3:
// 
// Input: s = "MCMXCIV"
// Output: 1994
// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
//  

// 1 <= s.length <= 15
// s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
// It is guaranteed that s is a valid roman numeral in the range [1, 3999].

use std::{collections::HashMap, hash::Hash};

fn main() {
    struct Solution{};
    impl Solution {
        pub fn roman_to_int(s: String) -> i32 {
            // 1. place all roman character in a HashMap with respective values
            // 2. replace pairs with single values
            // 3. iterate over characters in the String
            // 4. Match each character to a hashmap key, and sum the value to a var
            // 5. return sum
            let mut roman_values: HashMap<&str, i32> = HashMap::new();
            let mut roman_sum: i32 = 0;

            roman_values.insert("I", 1);
            roman_values.insert("V", 5);
            roman_values.insert("X", 10);
            roman_values.insert("L", 50);
            roman_values.insert("C", 100);
            roman_values.insert("D", 500);
            roman_values.insert("M", 1000);

            let mut cloned_str = s.clone();

            cloned_str = cloned_str
                .replace("IV", "IIII")
                .replace("IX", "VIIII")
                .replace("XL", "XXXX")
                .replace("XC", "LXXXX")
                .replace("CD", "CCCC")
                .replace("CM", "DCCCC");

            for (idx, r_char) in cloned_str.chars().enumerate() {
                // println!("character: {}", r_char);

                let roman_val = roman_values.get(&r_char.to_string() as &str);
                if let Some(&val) = roman_val{
                    roman_sum += val;
                }
            }

            // println!("Sum: {}", roman_sum);
            return roman_sum;
        }
    }

    Solution::roman_to_int(String::from("II")); // should return 2
    Solution::roman_to_int(String::from("LVIII"));// should return 58
    Solution::roman_to_int(String::from("MCMXCIV")); // should return 1994

}
