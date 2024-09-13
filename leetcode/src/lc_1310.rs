//! lc_1310
/*
    Leetcode 1310: XOR Queries of a subarray

    You are given an array arr of positive integers. You are also given the array queries where queries[i] = [lefti, righti].

    For each query i compute the XOR of elements from lefti to righti (that is, arr[lefti] XOR arr[lefti + 1] XOR ... XOR arr[righti] ).

    Return an array answer where answer[i] is the answer to the ith query.
*/

pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    // Step 1: Compute the prefix XOR array
    let mut prefix = vec![0; arr.len() + 1];
    for i in 0..arr.len() {
        prefix[i + 1] = prefix[i] ^ arr[i];
    }

    // Step 2: Process queries
    queries
        .into_iter()
        .map(|query| {
            let start = query[0] as usize;
            let end = query[1] as usize;
            prefix[end + 1] ^ prefix[start]
        })
        .collect()
}

pub fn run_xor_queries() {
    let mut arr: Vec<i32> = Vec::new();
    let mut queries: Vec<Vec<i32>> = Vec::new();

    arr.push(1);
    arr.push(3);
    arr.push(4);
    arr.push(8);

    queries.push(vec![0, 1]);
    queries.push(vec![1, 2]);
    queries.push(vec![0, 3]);
    queries.push(vec![3, 3]);
    println!("arr is {:?}", arr);
    println!("queries are {:?}", queries);
    let res = xor_queries(arr, queries);
    println!("res is {:?}", res);
}
