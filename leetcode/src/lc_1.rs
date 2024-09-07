//! Leetcode 1 : Two Sum | Easy
//
//
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//
//You may assume that each input would have exactly one solution, and you may not use the same element twice.
// 
// You can return the answer in any order.

use std::collections::HashMap;

// Function to find two indices in a vector that sum up to the target
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_to_index = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = num_to_index.get(&complement) {
            return vec![index as i32, i as i32];
        }
        num_to_index.insert(num, i);
    }
    vec![]
}

// Function to execute the problem and test cases
pub fn run_two_sum_example() {
    // Test case 1
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("Result for nums = [2, 7, 11, 15], target = 9: {:?}", result);

    // Test case 2
    let nums = vec![3, 2, 4];
    let target = 6;
    let result = two_sum(nums, target);
    println!("Result for nums = [3, 2, 4], target = 6: {:?}", result);

    // Test case 3
    let nums = vec![3, 3];
    let target = 6;
    let result = two_sum(nums, target);
    println!("Result for nums = [3, 3], target = 6: {:?}", result);

    // Test case 4 (no solution)
    let nums = vec![1, 2, 3];
    let target = 7;
    let result = two_sum(nums, target);
    println!("Result for nums = [1, 2, 3], target = 7: {:?}", result);
}
