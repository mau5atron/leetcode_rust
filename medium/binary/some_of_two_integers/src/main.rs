// LeetCode #371: Sum of Two Integers (binary)

// Given two integers a and b, return the sum of the 
// two integers without using the operators + and -.


// Example 1:
// Input: a = 1, b = 2
// Output: 3


// Example 2:
// Input: a = 2, b = 3
// Output: 5
 

// Constraints:

// -1000 <= a, b <= 1000

// Solution: Use 'XOR' and 'And' for bit manipulation

struct Solution;
impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = a ^ b;
            let carry = (a & b) << 1;
            a = temp;
            b = carry;
        }

        return a;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn test1(){
        let a = 1;
        let b = 2;

        // a in binary is 00000001
        // b in binary is 00000010
        assert_eq!(Solution::get_sum(a, b), 3);
    }

    #[test]
    pub fn test2(){
        let a = 2;
        let b = 3;

        // a in binary is 00000010
        // b in binary is 00000011

        assert_eq!(Solution::get_sum(a, b), 5);
    }

    #[test]
    pub fn test3(){
        let a = 5;
        let b = 3;

        // a = 00000101
        // b = 00000011
        assert_eq!(Solution::get_sum(a, b), 8);
    }
}

fn main() {
}
