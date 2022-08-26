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
use std::{option, ptr::null, any, thread::current};

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

// struct Helper{}
// impl Helper {
//     fn unwrapped_option_box_listnode(node: &Option<Box<ListNode>>) -> &ListNode {
//         match node.as_deref() {
//             Some(s_node) => {
//                 let mut some_node = node.as_ref().unwrap().as_ref();
//                 return some_node;
//             },
//             None => print!("ya")
//         }
//     }
// }

impl Solution {
  /*   fn unwrapped_option_box_listnode(node: &Option<Box<ListNode>>) -> &ListNode {
        let mut some_node = node.as_ref().unwrap().as_ref();
        return some_node;
    } */

    // able to safely traverse the linked list
   /*  fn traverse_linked_lists(mut dummy_node: ListNode, mut list1: &Option<Box<ListNode>>, mut list2: &Option<Box<ListNode>>){
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
    } */

    fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        let mut result_list = Box::new(ListNode::new(0)); // initialize with dummy node at the head
        let mut current_head = &mut result_list; // mutable reference to result_list, will be mutating result_list
        // from current_head

        let mut list1_current = list1; // copy head address from list1 to list1_current
        let mut list2_current = list2; // copy head address from list2 to list2_current

        while list1_current.is_some() && list2_current.is_some() {
            let mut l1_taken = list1_current.take(); // take copied list1 address
            let mut l2_taken = list2_current.take(); // take copied list2 address
            
            if let (Some(mut boxed_l1_head), Some(mut boxed_l2_head)) = (l1_taken, l2_taken){
                if boxed_l1_head.val <= boxed_l2_head.val {

                    list1_current = boxed_l1_head.next.take(); // take the next node from the copied address, and assign back to list1_current
                    // so it is ready on next iteration
                    list2_current = Some(boxed_l2_head); // list2_current does not change from when we took its address, assign list2_current
                    // its previous address when l1_taken = list1_current.take()
                    // address from list2_current.take
                    current_head = current_head.next.get_or_insert(boxed_l1_head);
                } else {

                    list2_current = boxed_l2_head.next.take(); // take the next node from the copied address and assign back to list2_current
                    // so it is ready on next iteration
                    list1_current = Some(boxed_l1_head); // list1_current does not change from when we took its address, assign list1_current
                    // its previous address when l2_taken = list2_current.take()
                    current_head = current_head.next.get_or_insert(boxed_l2_head);
                }
            }
        }

        if list1_current.is_some(){
            current_head.next = list1_current; // assign last list1_current node to current_head.next
        }

        if list2_current.is_some(){
            current_head.next = list2_current; // assign last list2_current node to current_head.next
        }

        return result_list.next;
    }
}


// Merge Two Sorted Lists

// I had a lot of trouble with this one, mainly borrow checker was throwing me off. 

// I ended up using someone else's solution that was clear enough to understand and added my own notes. The trick was to create a mutable reference pointing to the dummy node initially, then create copies of the memory addresses pointing to the head of each list being passed into the function and use those to mutate, rather than try and borrow each input list further down into the while loop.

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
    let mut node1_next_next = ListNode::new(3);

    node1_next.next = Some(Box::new(node1_next_next));
    let mut node1 = ListNode::new(1);
    node1.next = Some(Box::new(node1_next));

    // 1, 3, 4
    let mut node2_next = ListNode::new(5);
    let mut node2_next_next = ListNode::new(6);
    node2_next.next = Some(Box::new(node2_next_next));
    let mut node2: ListNode = ListNode::new(4);
    node2.next = Some(Box::new(node2_next));

    let list_one = Some(Box::new(node1));
    let list_two = Some(Box::new(node2));
    Solution::merge_two_lists(list_one, list_two);
    
}
