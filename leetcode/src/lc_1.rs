//! Leetcode 1 : Two Sum | Easy
//
//
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//
//You may assume that each input would have exactly one solution, and you may not use the same element twice.
// 
// You can return the answer in any order.



// Function to find two indices in a vector that sum up to the target
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    
    None
}

pub fn run_two_sum_example() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    match two_sum(&nums, target) {
        Some(indices) => println!("Indices: {:?}", indices),
        None => println!("No two sum solution found."),
    }

    // Additional test cases
    let nums = vec![3, 2, 4];
    let target = 6;
    match two_sum(&nums, target) {
        Some(indices) => println!("Indices: {:?}", indices),
        None => println!("No two sum solution found."),
    }
}