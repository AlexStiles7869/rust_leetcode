use std::collections::HashSet;

struct Solution {}

// Solution implementation
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // Add each of the elements in the vector to the hash set
        let vec_set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
 
        // Compare the length of the hash set to the array. If they aren't then there are duplicate
        // elements.
        !(vec_set.len() == nums.len())
    }
}

// Main function
fn main() {
    println!("{}", Solution::contains_duplicate(vec![1,2,3,1]));
}