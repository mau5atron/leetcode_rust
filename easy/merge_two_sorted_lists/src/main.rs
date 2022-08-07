// You are given the heads of two sorted linked lists list1 and& list2.

// Merge the two lists in a one sorted list. 
// The list should be made by splicing together the nodes of the first two lists.

// Return the head of the merged linked list.

 

// Example 1:

// Input: list1 = [1,2,4], list2 = [1,3,4]
// Output: [1,1,2,3,4,4]

// -----------------------------------------

// Example 2:

// Input: list1 = [], list2 = []
// Output: []

// -----------------------------------------

// Example 3:

// Input: list1 = [], list2 = [0]
// Output: [0]
 

// Constraints:

// The number of nodes in both lists is in the range [0, 50].
// -100 <= Node.val <= 100
// Both list1 and list2 are sorted in non-decreasing order.
#![allow(unused)]
use std::{option, ptr::null, any};

pub struct Solution {}

struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val: val,
            next: None
        }
    }
}

struct Helper{}
impl Helper {
    fn unwrapped_option_box_listnode(node: &Option<Box<ListNode>>) -> &ListNode {
        match node.as_deref() {
            Some(s_node) => {
                let mut some_node = node.as_ref().unwrap().as_ref();
                return some_node;
            },
            None => print!("ya")
        }
    }
}

impl Solution {
  /*   fn unwrapped_option_box_listnode(node: &Option<Box<ListNode>>) -> &ListNode {
        let mut some_node = node.as_ref().unwrap().as_ref();
        return some_node;
    } */

    // able to safely traverse the linked list
    fn traverse_linked_lists(mut dummy_node: ListNode, mut list1: &Option<Box<ListNode>>, mut list2: &Option<Box<ListNode>>){
        println!("current node val1: {}, current node val2: {}\n", Helper::unwrapped_option_box_listnode(list1).val, Helper::unwrapped_option_box_listnode(list2).val);
        if (
            Helper::unwrapped_option_box_listnode(list1).next.is_none() &&
            Helper::unwrapped_option_box_listnode(list2).next.is_none()
        ){
            return;
        }

        if(Helper::unwrapped_option_box_listnode(list1).val < Helper::unwrapped_option_box_listnode(list2).val){
            let mut node1_val = Helper::unwrapped_option_box_listnode(list1).val;
            let mut node1_new = ListNode::new(node1_val);
            // dummy_node.next = Some(Box::new())
            dummy_node.next = Some(Box::new(node1_new));
            list1 = &list1.as_ref().unwrap().next;

            print!("hello");
        }

        Solution::traverse_linked_lists(dummy_node, &Helper::unwrapped_option_box_listnode(list1).next, &Helper::unwrapped_option_box_listnode(list2).next);
    }

    fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_node = ListNode::new(-1); // just return this as Option<Box<ListNode>> at the end
        // let mut head_node: ListNode = dummy_node;

        let mut list_node_pair = Some((&list1, &list2));
        
        if let Some((first_list_head, second_list_head)) = list_node_pair.take(){
            // let mut node = first_inner;
            // println!("node.value: {}", node.unwrap().val);
            // let mut first_head = first_list_head.as_ref().unwrap().as_ref();
            println!("first_inner value: {}", first_list_head.as_ref().unwrap().val);
            // println!("first_inner next value: {}", first_list_head.as_ref().unwrap().next.as_ref().unwrap().val);
     /*        match Solution::unwrapped_option_box_listnode(first_list_head).next.as_deref() {
                Some(node_val) => println!("Node val!: {}", node_val.val),
                None => println!("no value :/")
            } */

            Solution::traverse_linked_lists(dummy_node, first_list_head, second_list_head);
            // looping here works! but unsafe, it is infinite loop
         /*    while !Solution::unwrapped_option_box_listnode(first_list_head).next.as_deref().is_none() {
                match Solution::unwrapped_option_box_listnode(first_list_head).next.as_deref() {
                    Some(node_val) => println!("Node val in loop : {}", node_val.val),
                    None => println!("sorry no node val :/")
                }
            } */
            
            
            // println!("second_inner value: {}", second_list_head.as_ref().unwrap().val);
        }


        return list1;
    }
}

fn main() {
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

    // node1 stuff
    // 1, 2, 4
    let mut node1_next = ListNode::new(2);
    // println!("Checking if none : {}", node1_next.next.is_none());
    let mut node1_next_next = ListNode::new(4);

    node1_next.next = Some(Box::new(node1_next_next));
    let mut node1 = ListNode::new(1);
    node1.next = Some(Box::new(node1_next));

    // 1, 3, 4
    let mut node2_next = ListNode::new(3);
    let mut node2_next_next = ListNode::new(4);
    node2_next.next = Some(Box::new(node2_next_next));
    let mut node2: ListNode = ListNode::new(1);
    node2.next = Some(Box::new(node2_next));

    let list_one = Some(Box::new(node1));
    let list_two = Some(Box::new(node2));
    Solution::merge_two_lists(list_one, list_two);
    
}
