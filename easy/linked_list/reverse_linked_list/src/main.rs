// LeetCode #206: Reverse Linked List

// Given the head of a singly linked list, 
// reverse the list, and return the reversed list.

// Example 1:
// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]

// Example 2:
// Input: head = [1,2]
// Output: [2,1]

// Constraints:

// The number of nodes in the list is the range [0, 5000].
// -5000 <= Node.val <= 5000



// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
#[derive(Debug)]
#[derive(PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // will do easy way first
        let mut prev_node: Option<Box<ListNode>> = None;
        let mut original_node: Option<Box<ListNode>> = head;

        while let Some(mut curr_node) = original_node{
            // println!("node.next before moving to head: {:?}", curr_node.next);
            original_node = curr_node.next; // advancing current node
            curr_node.next = prev_node; // previous node will hold on to the node from previous
            // iteration
            prev_node = Some(curr_node);
            // println!("prev_node at end: {:?}\n\n\n", prev_node);
        }

        return prev_node;
    }
}

fn to_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current_node = None;
    for &value in list.iter().rev(){
        let mut new_node = ListNode::new(value);
        new_node.next = current_node;
        current_node = Some(Box::new(new_node));
    }

    return current_node;
}

fn main() {
    let vec_list = vec![1, 2, 3, 4];
    // println!("{:?}", to_list(vec_list));

    Solution::reverse_list(to_list(vec_list));
}

#[cfg(test)]
mod tests {
    use crate::{to_list, Solution};

    #[test]
    pub fn test1(){
        let mut input_list = to_list(vec![1,2,3,4,5]);
        let mut expected_result = to_list(vec![5, 4, 3, 2, 1]);

        assert_eq!(Solution::reverse_list(input_list), expected_result);
    }

    #[test]
    pub fn test2(){
        let mut input_list = to_list(vec![1,2]);
        let mut expected_result = to_list(vec![2, 1]);

        assert_eq!(Solution::reverse_list(input_list), expected_result);
    }
}
