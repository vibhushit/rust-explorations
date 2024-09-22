//! Leetcode 440 | Kth smallest in Lexographical Order

/*

Given two integers n and k, return the kth lexicographically smallest integer in the range [1, n].

Example 1:
Input: n = 13, k = 2
Output: 10
Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.


Example 2:
Input: n = 1, k = 1
Output: 1


Constraints:
1 <= k <= n <= 109

*/

// use std::collections::{binary_heap, BinaryHeap};

pub fn find_kth_number(n: i32, k: i32) -> i32 {
    let mut curr = 1;
    let mut k = k - 1; // since we're starting from 1, we decrement k by 1

    while k > 0 {
        let steps = count_steps(curr, n);
        if steps <= k {
            // If the number of steps from `curr` to `curr+1` is less than or equal to k,
            // move to the next sibling (curr+1)
            curr += 1;
            k -= steps;
        } else {
            // Otherwise, dive deeper into the tree (append 0 to curr)
            curr *= 10;
            k -= 1;
        }
    }

    curr
}

pub fn count_steps(curr: i32, n: i32) -> i32 {
    let mut steps = 0;
    let mut first = curr as i64;
    let mut last = curr as i64;
    let n = n as i64;

    while first <= n {
        steps += std::cmp::min(last, n) - first + 1;
        first *= 10;
        last = last * 10 + 9;
    }

    steps as i32
}

pub fn run_find_kth_number() {
    let n = 13;
    let k = 2;

    let res = find_kth_number(n, k);
    println!("res is {}", res);
}
