// LeetCode # 70

// You are climbing a staircase. It takes n steps to reach the top.

// Each time you can either climb 1 or 2 steps. 
// In how many distinct ways can you climb to the top?

 

// Example 1:

// Input: n = 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps



// Example 2:

// Input: n = 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step
 

// Constraints:

// 1 <= n <= 45

struct Sol {}

impl Sol {
    pub fn climb_stairs(n: i32) -> i32 {
        if (n == 1){
            return 1;
        }

        let mut one = 1;
        let mut two = 2;

        let mut i = 3;
        while i <= n {
            let total = one + two;
            one = two;
            two = total;
            i += 1;
        }
        
        return two;
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use crate::Sol;

    #[test]
    pub fn test1(){
        let expected_result = 2;
        let n = 2;
        assert_eq!(Sol::climb_stairs(n), expected_result);
    }

    #[test]    
    pub fn test2(){
        let expected_result = 3;
        let n = 3;
        assert_eq!(Sol::climb_stairs(n), expected_result);
    }

    #[test]
    pub fn test3(){
        let expected_result = 8;
        let n = 5;
        assert_eq!(Sol::climb_stairs(n), expected_result);
    }
}