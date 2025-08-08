use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create hash map containing complements required for each element in vector to meet
        // target.
        let complement_map: HashMap<i32, i32> = HashMap::from_iter(
            nums.iter()
                .enumerate()
                .map(|(i, num)| (target - num, i as i32)),
        );

        // Iterate over the nums vector again and see if any num has a valid complement in the hash
        // map.
        let mut result: Vec<i32> = vec![];
        for (i, num) in nums.iter().enumerate() {
            if complement_map.contains_key(num) {
                // Get candidate
                let candidate: i32 = *complement_map.get(num).unwrap();

                // Ensure candidate isn't the same as the currently evaluated index
                if candidate != (i as i32) {
                    result = vec![i as i32, candidate];
                    break;
                }
            }
        }

        // Return the result
        result
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
}
