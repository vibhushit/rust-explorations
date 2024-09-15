//!  Leetcode 2419 | Longest Subarray with Maximum bitwise AND

/*
    You are given an integer array nums of size n.

    Consider a non-empty subarray from nums that has the maximum possible bitwise AND.

    In other words, let k be the maximum value of the bitwise AND of any subarray of nums. Then, only subarrays with a bitwise AND equal to k should be considered.
    Return the length of the longest such subarray.

    The bitwise AND of an array is the bitwise AND of all the numbers in it.

    A subarray is a contiguous sequence of elements within an array.

*/

pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut max_occurance = 1;
    let mut val = nums[0];
    let mut adjacent_occurance = 0;

    for num in nums {
        if num == val {
            adjacent_occurance += 1;
            max_occurance = std::cmp::max(adjacent_occurance, max_occurance);
        } else if num > val {
            val = num;
            adjacent_occurance = 1;
            max_occurance = 1;
        } else {
            adjacent_occurance = 0;
        }
    }
    max_occurance
}

pub fn run_longest_subarray() {
    // let nums = vec![1,2,3,3,2,2];
    // let nums = vec![323376,323376,323376,323376,323376,323376,323376];
    let nums = vec![
        311155, 311155, 311155, 311155, 311155, 311155, 311155, 311155, 201191, 311155,
    ];

    let res = longest_subarray(nums);
    println!("max value is {}", res);
}
