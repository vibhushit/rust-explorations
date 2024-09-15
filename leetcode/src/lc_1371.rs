//! Leetcode 1371 | Find the Longest Substring Containing Vowels in Even Counts

/*
    Given the string s, return the size of the longest substring containing each vowel an even number of times.
    That is, 'a', 'e', 'i', 'o', and 'u' must appear an even number of times.
*/

use std::collections::HashMap;

pub fn find_the_longest_substring(s: String) -> i32 {
    let mut longest_substring = 0;
    let mut vowel_state = 0u8;
    let mut state_first_seen: HashMap<u8, i32> = HashMap::new();
    state_first_seen.insert(0, -1);

    for (index, ch) in s.chars().enumerate() {
        vowel_state ^= match ch {
            'a' => 1 << 0,
            'e' => 1 << 1,
            'i' => 1 << 2,
            'o' => 1 << 3,
            'u' => 1 << 4,
            _ => 0,
        };

        let current_index = index as i32;
        longest_substring = longest_substring.max(
            current_index
                - state_first_seen
                    .get(&vowel_state)
                    .copied()
                    .unwrap_or(current_index),
        );

        state_first_seen.entry(vowel_state).or_insert(current_index);
    }

    longest_substring
}

pub fn run_find_longest_substring() {
    let s = String::from("leetcodeisgreat");
    let len = find_the_longest_substring(s);
    println!("Size of longest substring is {len}");
}
