//! Leetcode 1684 | Count the number of consistent strings

/* 
    You are given a string allowed consisting of distinct characters and an array of strings words. 
    A string is consistent if all characters in the string appear in the string allowed.

    Return the number of consistent strings in the array words.
*/

use std::collections::HashSet;

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let set : HashSet<char> = allowed.chars().collect();
    words.iter()
        .filter(|word| word.chars().all(|ch| set.contains(&ch)))
        .count() as i32
}

pub fn run_count_number_of_consistent_strings(){
    let allowed = String::from("ab");
    let mut words : Vec<String> = Vec::new();

    words.push("ad".to_string());
    words.push("bd".to_string());
    words.push("aaab".to_string());
    words.push("baa".to_string());
    words.push("badab".to_string());

    let count = count_consistent_strings(allowed, words);
    println!("Count of consistent strings is {}", count);
}