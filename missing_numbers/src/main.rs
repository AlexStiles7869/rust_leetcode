// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/description/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn missing_numbers(nums: Vec<i32>) -> Vec<i32> {
        // Create hash set from vector elements -> O(n)
        let nums_set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());

        // Filter and return numbers in vector that aren't in hash map (and therefore aren't
        // present)
        (1..=nums.len() as i32).filter(|num| !nums_set.contains(num)).collect()
    
    }

    pub fn missing_numbers_2(nums: Vec<i32>) -> Vec<i32> {
        let mut seq: Vec<i32> = [0].repeat(nums.len());
        nums.into_iter().for_each(|num| seq[(num-1) as usize] += 1);
        seq.into_iter().enumerate().filter(|&(x, y)| y == 0).map(|(x, y)| (x+1) as i32).collect::<Vec<i32>>()
    }
}

fn main() {
    println!("{:?}", Solution::missing_numbers_2(vec![4,3,2,7,8,2,3,1]));
}
