// LeetCode #1143: Longest Common Subsequence

// Given two strings text1 and text2, return the length of 
// their longest common subsequence. If there is no common 
// subsequence, return 0.

// A subsequence of a string is a new string generated from the 
// original string with some characters (can be none) deleted 
// without changing the relative order of the remaining characters.

// For example, "ace" is a subsequence of "abcde".
// A common subsequence of two strings is a subsequence that is common 
// to both strings.

 

// Example 1:
// Input: text1 = "abcde", text2 = "ace" 
// Output: 3  
// Explanation: The longest common subsequence is "ace" and its length is 3.


// Example 2:
// Input: text1 = "abc", text2 = "abc"
// Output: 3
// Explanation: The longest common subsequence is "abc" and its length is 3.

// Example 3:
// Input: text1 = "abc", text2 = "def"
// Output: 0
// Explanation: There is no such common subsequence, so the result is 0.

// Constraints:

// 1 <= text1.length, text2.length <= 1000
// text1 and text2 consist of only lowercase English characters.


// public int longestCommonSubsequence(String text1, String text2) {
        
//     int [][] matrix = new int[text1.length()+1] [text2.length() + 1];
    
//     for(int j = text2.length()-1 ; j>=0 ; j--){
//         for(int i = text1.length()-1 ; i>=0 ; i--){
//             if(text1.charAt(i) == text2.charAt(j)){
//                 matrix[i][j] = 1+matrix[i+1][j+1];
//             }
//             else{
//                 matrix[i][j]= Math.max(matrix[i+1][j], matrix[i][j+1]);
//             }
//         }
//     }
    
//     return matrix[0][0];
// }




struct Sol {}

impl Sol {

    // O(m*n) this does not run efficiently for some reason
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // let mut matrix: Vec<i32>, Vec<i32> = vec![];
        let matrix_width = text1.len() + 1;
        let matrix_height = text2.len() + 1;

        if text1.len() < text2.len() {
            return self::Sol::longest_common_subsequence(text2, text1)
        }

        // matrix width is the col size
        // matrix height is the row size
        let mut matrix = vec![vec![0; matrix_width]; matrix_height];
        // println!("Full matrix: {:?}", matrix);
        let mut j = (text2.len() - 1) as i32;
        while j >= 0 { // j is row
            // reset column i back to original text1 size - 1
            let mut i = (text1.len() - 1) as i32;
            while i >= 0 { // i is col
                // abcde -> ace, 1 & 2
                if text1.chars().nth(i as usize).unwrap() == text2.chars().nth(j as usize).unwrap(){
                    // println!("if i={}, j={}", i, j);
                    // matrix[i][j] = 1+matrix[i+1][j+1]; // this fails
                    matrix[j as usize][i as usize] = 1 + matrix[(j+1) as usize][(i+1) as usize];
                } else {
                    // println!("else i={}, j={}", i, j);
                    matrix[j as usize][i as usize] = std::cmp::max(matrix[(j+1) as usize][i as usize], matrix[j as usize][(i+1) as usize]);
                }

                i-=1;
            }

            j-=1;
        }

        // println!("end matrix: {:?}", matrix);
        return matrix[0][0];
    }


    // O(n*m) 4ms, 2.1 MB
    fn longest_common_subsequence2(text1: String, text2: String) -> i32 {
        if text1.len() < text2.len(){
            return self::Sol::longest_common_subsequence2(text2, text1);
        }

        let len1 = text1.len();
        let len2 = text2.len();
        let string1: Vec<char> = text1.chars().collect();
        let string2: Vec<char> = text2.chars().collect();
        let mut matrix = vec![vec![0; len2+1]; 2]; // make outer only size 2
        // println!("matrix init: {:?}", matrix);

        for i in 1..len1 + 1 { // row
            for j in 1..len2 + 1 { // col
                // println!("current_matrix: {:?}", matrix);
                if string1[i-1] == string2[j-1] {
                    // println!("first i = {}, j = {}, i%2 = {}", i, j, i%2);
                    matrix[i%2][j] = matrix[(i-1)%2][j-1] + 1;
                } else {
                    // println!("second i = {}, j = {}, i%2 = {}", i, j, i%2);
                    matrix[i%2][j] = std::cmp::max(matrix[(i-1)%2][j], matrix[i%2][j-1]);
                }
            }
        }

        // println!("len1%2 = {}, len1 = {}, len2 = {}", len1%2, len1, len2);
        println!("final matrix: {:?}", matrix);
        return matrix[len1%2][len2];
    }
}

fn main() {
    // let mut some_vec: Vec<i32> = vec![1, 2, 3, 4];
    // println!("iter list: {:?}", some_vec.iter().map(|&x| x * x).collect::<Vec<i32>>());
    let some_text: String = String::from("hello");
    some_text.chars().nth(0).unwrap();
    println!("some_text = {}", some_text);
    
}

#[cfg(test)]
mod tests {
    use crate::Sol;

    #[test]
    pub fn test1(){
        // Input: text1 = "abcde", text2 = "ace"
        // Output: 3
        let text1: String = String::from("abcde");
        let text2: String = String::from("ace");
        assert_eq!(Sol::longest_common_subsequence2(text1, text2), 3);
    }
    #[test]
    pub fn test2(){
        // Input: text1 = "abc", text2 = "abc"
        // Output: 3
        let text1: String = String::from("abc");
        let text2: String = String::from("abc");
        assert_eq!(Sol::longest_common_subsequence2(text1, text2), 3);
    }
    #[test]
    pub fn test3(){
        // Input: text1 = "abc", text2 = "def"
        // Output: 0
        let text1: String = String::from("abc");
        let text2: String = String::from("def");
        assert_eq!(Sol::longest_common_subsequence2(text1, text2), 0);
    }
}