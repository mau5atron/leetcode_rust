
#![allow(unused)]
// also called balanced brackets

// Given a&& string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
 

// Example 1:

// Input: s = "()"
// Output: true
// Example 2:

// Input: s = "()[]{}"
// Output: true
// Example 3:

// Input: s = "(]"
// Output: false
 

// Constraints:

// 1 <= s.length <= 104
// s consists of parentheses only '()[]{}'.

use std::{collections::HashMap, fmt::Debug, string};

fn main() {
    struct Solution {};
    impl Solution {
        pub fn is_valid(s: String) -> bool {
            // This solution time complexity TC -> O(n) just iterating on a string
            // Solution Space Complexity -> O(n) due to stack data structure

            // push front bracket to stack
            // if first iteration, stack is_empty() then return false
            // continue iterating on string and pushing to stack if any front bracket is encountered

            // if through iteration we encounter an ending bracket, we pop off the last element 
            // in the stack and try to match with current ending bracket

            // if they match, keep going until the stack is empty or there is a bracket mismatch
            // if the stack is empty at the end, the brackets are balanced
            if s.len() % 2 != 0 || s.len() == 0 {
                return false;
            }

            let mut front_bracket_stack: Vec<char> = Vec::new();

            for (idx, str_ch) in s.chars().enumerate() {
                // println!("Char: {}", str_ch);
                if (str_ch == '(' || str_ch == '[' || str_ch == '{'){
                    // println!("being pushed in: {}", str_ch);
                    front_bracket_stack.push(str_ch);
                } else {
                    // println!("s = {}", str_ch);
                    if front_bracket_stack.is_empty(){
                        return false;
                    }

                    let c: Option<char> = front_bracket_stack.pop();
                    if let Some(front_bracket_val) = c {
                        println!("front_bracket_val == {:?}", front_bracket_val);
                        if (front_bracket_val == '(' && str_ch == ')') 
                        || (front_bracket_val == '[' && str_ch == ']') 
                        || (front_bracket_val == '{' && str_ch == '}') {
                            println!("{} and {} match, ", front_bracket_val, str_ch);
                        } else {
                            print!("{} and {} do not match, ", front_bracket_val, str_ch);
                            return false;
                        }
                    }
                }
            }
            
            if front_bracket_stack.is_empty() {
                return true;
            } else {
                return false;
            }

            return true;
        }

        pub fn is_valid2(s: String) -> bool {
            let mut bracket_pairs_map: HashMap<char, char> = HashMap::from([
                ('(', ')'),
                ('[', ']'),
                ('{', '}'),
            ]);
         /*    bracket_pairs_map.insert('(', ')');
            bracket_pairs_map.insert('[', ']');
            bracket_pairs_map.insert('{', '}'); */
            let mut front_bracket_stack: Vec<char> = Vec::new();

            for (idx, string_char) in s.chars().enumerate(){
                // checking to see if current iterated char value from str is present in hashmap
                if bracket_pairs_map.contains_key(&string_char){
                    front_bracket_stack.push(string_char); // push front bracket to stack
                } else {
                    if let Some(val) = front_bracket_stack.pop(){
                        if let Some(k) = bracket_pairs_map.get_key_value(&val){
                            println!("k_val: {:?}\n", k.1);
                            println!("str_char: {}\n", string_char);
                            if k.1 != &string_char || front_bracket_stack.is_empty() {
                                return false;
                            }
                        }
                        // key (left), value (right) bracket
                        // get the value of key(val) from hashmap
                        // and then compare to string_char
                    }
                    return false;
                } 
            }

            if front_bracket_stack.is_empty() {
                return true;
            } else {
                return false;
            }

            return true;
        }

        pub fn test_cases_first_solution(){
            let mut prob1 = Solution::is_valid(String::from("()"));
            println!("prob1 is: {}\n", prob1);
        
            let mut prob2: bool = Solution::is_valid(String::from("()[]{}"));
            println!("prob2 is: {}\n", prob2);
        
            let mut prob3: bool = Solution::is_valid(String::from("(]"));
            println!("prob3 is: {}\n", prob3);
        
            let mut prob4: bool = Solution::is_valid(String::from("["));
            println!("prob4 is: {}\n", prob4);
        
            let mut prob5: bool = Solution::is_valid(String::from("()"));
            println!("prob5 is: {}\n", prob5);
        
            let mut prob6: bool = Solution::is_valid(String::from("]"));
            println!("prob6 is: {}\n", prob6);
        }

        pub fn test_cases_second_solution(){
            let prob1 = Self::is_valid2(String::from("()"));
            println!("prob1 is : {}", prob1);
        }
    };

    // Solution::test_cases_second_solution();
    Solution::test_cases_first_solution();
}
