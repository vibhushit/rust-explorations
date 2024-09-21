//! Leetcode 386 | Lexicographical Numbers

/*
Given an integer n, return all the numbers in the range [1, n] sorted in lexicographical order.
You must write an algorithm that runs in O(n) time and uses O(1) extra space.

Example 1:

Input: n = 13
Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]

Example 2:

Input: n = 2
Output: [1,2]

Constraints:

1 <= n <= 5 * 104

*/

pub fn leixcal_order(n: i32) -> Vec<i32> {
    // Create a mutable vector to store the result
    let mut result = Vec::new();

    // Define a helper function for DFS (depth-first search)
    // It takes the current number as input and recursively explores the next lexicographical numbers
    fn dfs(current: i32, n: i32, result: &mut Vec<i32>) {
        // If the current number exceeds n, we stop further exploration
        if current > n {
            return;
        }

        // Add the current number to the result vector
        result.push(current);

        // Try to append digits 0 to 9 to the current number to form the next lexicographical numbers
        // We only append if the resulting number <= n
        for i in 0..10 {
            let next_num = current * 10 + i;
            // If the next number exceeds n, we stop and return to the previous call (backtrack)
            if next_num > n {
                break;
            }
            // Recursively call dfs on the next number
            dfs(next_num, n, result);
        }
    }

    // Start the DFS from each number in the range 1 to 9
    // These are the "roots" of the lexicographical tree
    for i in 1..10 {
        // If i exceeds n, there's no point in exploring further
        if i > n {
            break;
        }
        // Call DFS starting from each of these numbers (1, 2, 3, ..., 9)
        dfs(i, n, &mut result);
    }

    // Return the lexicographically sorted result
    result
}

pub fn run_lexical_order() {
    let n = 300;

    let res = leixcal_order(n);

    println!("res is :\n {res:?}");
}
