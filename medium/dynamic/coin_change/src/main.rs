// LeetCode # 322: Coin Change

// You are given an integer array coins representing coins of 
// different denominations and an integer amount representing 
// a total amount of money.

// Return the fewest number of coins that you need to make up 
// that amount. If that amount of money cannot be made up by any 
// combination of the coins, return -1.

// You may assume that you have an infinite number of each kind of coin.

 

// Example 1:

// Input: coins = [1,2,5], amount = 11
// Output: 3
// Explanation: 11 = 5 + 5 + 1



// Example 2:

// Input: coins = [2], amount = 3
// Output: -1



// Example 3:

// Input: coins = [1], amount = 0
// Output: 0
 

// Constraints:

// 1 <= coins.length <= 12
// 1 <= coins[i] <= 231 - 1
// 0 <= amount <= 104

struct Sol {}

impl Sol {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut amt: Vec<i32> = Vec::from(vec![amount + 1]);
        amt.fill(amount+1);

        amt[0] = 0;
        let mut i: usize = 1;
        while i < amount as usize {
            let mut j = 0;
            while j < coins.len(){

                if i >= coins[j] as usize{
                    amt[i] = std::cmp::min(amt[i], 1+amt[i-(coins[j] as usize)]);
                }

                j+=1;
            }
            i+=1;
        }

        if amt[amount as usize] < amount+1 {
            return amt[amount as usize];
        }

        return -1;
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use crate::Sol;

    
    #[test]
    pub fn test1(){
        let expected = 3;
        let coins = Vec::from(vec![1,2,5]);
        let amount = 11;

        assert_eq!(Sol::coin_change(coins, amount), expected);
    }


    #[test]
    pub fn test2(){
        let expected = -1;
        let coins = Vec::from(vec![2]);
        let amount = 3;

        assert_eq!(Sol::coin_change(coins, amount), expected);
    }


    #[test]
    pub fn test3(){
        let expected = 0;
        let coins = Vec::from(vec![1]);
        let amount = 0;

        assert_eq!(Sol::coin_change(coins, amount), expected);
    }

}