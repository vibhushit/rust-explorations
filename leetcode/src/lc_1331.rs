//! Rank Transform of an Array

/*
Given an array of integers arr, replace each element with its rank.

The rank represents how large the element is. The rank has the following rules:

Rank is an integer starting from 1.
The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
Rank should be as small as possible.

*/

use std::collections::HashMap;

pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    // Create a sorted version of the input array and remove duplicates
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    sorted_arr.dedup();

    // Create a hash map to store the rank of each unique element
    let mut rank_map = HashMap::new();
    for (rank, &val) in sorted_arr.iter().enumerate() {
        rank_map.insert(val, (rank + 1) as i32);
    }

    // Transform the original array into the ranked array
    arr.into_iter().map(|x| rank_map[&x]).collect()
}

pub fn run_array_rank_transform(){
    // let arr = vec![40,10,30,20];
    // let arr = vec![100,100,100];
    let arr = vec![37,12,28,9,100,56,80,5,12];

    
    let res = array_rank_transform(arr);

    println!("res is {:?}",res);
}