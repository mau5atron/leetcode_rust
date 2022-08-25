// LeetCode 953: Verifying an Alien Dictionary


// In an alien language, surprisingly, they also use English lowercase letters, but possibly in a different order. 
// The order of the alphabet is some permutation of lowercase letters.

// Given a sequence of words written in the alien language, and the order of the alphabet, 
// return true if and only if the given words are sorted lexicographically in this alien language.
 


// Example 1:

// Input: words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
// Output: true
// Explanation: As 'h' comes before 'l' in this language, then the sequence is sorted.



// Example 2:

// Input: words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
// Output: false
// Explanation: As 'd' comes after 'l' in this language, then words[0] > words[1], hence the sequence is unsorted.



// Example 3:

// Input: words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
// Output: false
// Explanation: The first three characters "app" match, and the second string is shorter (in size.) 
// According to lexicographical rules "apple" > "app", because 'l' > '∅', where '∅' is defined as the blank c
// haracter which is less than any other character (More info).

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut alpha_order_map: HashMap<char, usize> = HashMap::new();
        // fill out hashmap with string order and each chars index

        for (idx, c) in order.chars().enumerate(){
            // println!("char: {}", c);
            alpha_order_map.insert(c, idx);
        }

        let mut i = 0;
        while i < words.len()-1 {
            let mut j = 0;
            while j < words[i].len() {
                if j >= words[i+1].len(){
                    return false;
                }

                
                if words[i].chars().nth(j) != words[i+1].chars().nth(j){
                    let borrowed_char_at_j: char = words[i].chars().nth(j).unwrap();
                    let borrowed_next_char_from_j: char = words[i+1].chars().nth(j).unwrap();

                    let current_letter = alpha_order_map.get(&borrowed_char_at_j).unwrap();
                    let next_letter = alpha_order_map.get(&borrowed_next_char_from_j).unwrap();

                    if next_letter < current_letter {
                        return false;
                    } else {
                        break;
                    }
                }

                j+=1;
            }

            i+=1;
        }

        return true;
    }
}
fn main() {
    /* println!("0x311 as integer: {}", 0x311 as i32);
    3x16^2 + 1x16^1 + 1x16^0 */
    // unrelated, just converting hexidecimal
    let str: String = String::from("hlabcdefgijkmnopqrstuvwxyz");
    let some_vec: Vec<String> = Vec::from(vec![String::from("hello"), String::from("leetcode")]);
    Solution::is_alien_sorted(some_vec, str);
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn test1(){
        let words: Vec<String> = Vec::from(vec![String::from("hello"), String::from("leetcode")]);
        let order: String = String::from("hlabcdefgijkmnopqrstuvwxyz");
        let expected_result: bool = true;
        assert_eq!(Solution::is_alien_sorted(words, order), expected_result);
    }

    #[test]
    pub fn test2(){
        // "word","world","row"
        let words: Vec<String> = Vec::from(vec![String::from("word"), String::from("world"), String::from("row")]);
        let order: String = String::from("worldabcefghijkmnpqstuvxyz");
        let expected_result: bool = false;
        assert_eq!(Solution::is_alien_sorted(words, order), expected_result);
    }

    #[test]
    pub fn test3(){
        // words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
        let words: Vec<String> = Vec::from(vec![String::from("apple"), String::from("app")]);
        let order: String = String::from("abcdefghijklmnopqrstuvwxyz");
        let expected_result: bool = false;
        assert_eq!(Solution::is_alien_sorted(words, order), expected_result);
    }
}
