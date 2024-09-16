//! Leetcode 539 | Minimum Time Difference
 
/*
    Given a list of 24-hour clock time points in "HH:MM" format, 
    return the minimum minutes difference between any two time-points in the list.
*/

struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        if time_points.len() > 1440 {
            return 0; // If more than 1440 time points, there must be a duplicate
        }
    
        let mut minutes: Vec<i32> = time_points.iter()
            .map(|t| t[..2].parse::<i32>().unwrap() * 60 + t[3..].parse::<i32>().unwrap())
            .collect();
    
        minutes.sort_unstable();
        minutes.push(minutes[0] + 1440); // Add first element + 1440 to handle circular nature
    
        minutes.windows(2)
            .map(|w| w[1] - w[0])
            .min()
            .unwrap()
    }
}

pub fn run_find_min_difference() {
    let mut time_points = Vec::new();

    time_points.push("00:00".to_string());
    time_points.push("23:00".to_string());
    time_points.push("10:20".to_string());
    time_points.push("09:00".to_string());
    time_points.push("00:00".to_string());

    let min_diff = Solution::find_min_difference(time_points);
    println!("min diff between two time points is {min_diff}");
}