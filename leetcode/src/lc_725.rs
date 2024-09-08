//! Leetcode 725 : Split Linked List in Parts

// Given the head of a singly linked list and an integer k, split the linked list into k consecutive linked list parts.
//
// The length of each part should be as equal as possible: no two parts should have a size differing by more than one. This may lead to some parts being null.
//
// The parts should be in the order of occurrence in the input list, and parts occurring earlier should always have a size greater than or equal to parts occurring later.
//
// Return an array of the k parts.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let mut length = 0;
    let mut current = head.as_ref();
    let mut parts = Vec::new();

    while let Some(node) = current {
        length += 1;
        current = node.next.as_ref();
    }

    let (base_size, mut extra) = (length / k, length % k);
    let mut current = head;

    for _ in 0..k {
        let mut part_size = base_size + if extra > 0 { 1 } else { 0 };
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut dummy;

        while part_size > 0 {
            tail.next = current.take();
            tail = tail.next.as_mut().unwrap();
            current = tail.next.take();
            part_size -= 1;
        }

        parts.push(dummy.next.take());
        if extra > 0 {
            extra -= 1;
        }
    }

    parts
}


pub fn run_split_list_to_parts() {
    // Helper function to create a linked list from a vector
    fn build_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &value in values.iter().rev() {
            let mut node = Box::new(ListNode::new(value));
            node.next = current;
            current = Some(node);
        }
        current
    }

    // Helper function to print a linked list
    fn print_linked_list(head: &Option<Box<ListNode>>) {
        let mut current = head.as_ref();
        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = node.next.as_ref();
        }
        println!("None");
    }

    // Test case 1: Linked list [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], k = 3
    let head = build_linked_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let result = split_list_to_parts(head, 3);
    println!("Test Case 1: k = 3");
    for (i, part) in result.iter().enumerate() {
        print!("Part {}: ", i + 1);
        print_linked_list(part);
    }
    println!("");

    // Test case 2: Linked list [1, 2, 3, 4, 5], k = 5
    let head = build_linked_list(vec![1, 2, 3, 4, 5]);
    let result = split_list_to_parts(head, 5);
    println!("Test Case 2: k = 5");
    for (i, part) in result.iter().enumerate() {
        print!("Part {}: ", i + 1);
        print_linked_list(part);
    }
    println!("");

    // Test case 3: Linked list [1, 2, 3, 4, 5], k = 7
    let head = build_linked_list(vec![1, 2, 3, 4, 5]);
    let result = split_list_to_parts(head, 7);
    println!("Test Case 3: k = 7");
    for (i, part) in result.iter().enumerate() {
        print!("Part {}: ", i + 1);
        print_linked_list(part);
    }
    println!("");

    // Test case 4: Empty linked list, k = 3
    let head = build_linked_list(vec![]);
    let result = split_list_to_parts(head, 3);
    println!("Test Case 4: k = 3 (Empty list)");
    for (i, part) in result.iter().enumerate() {
        print!("Part {}: ", i + 1);
        print_linked_list(part);
    }
    println!("");
}