struct Solution {}

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        // Simple approach, O(n^2)
        // Iterate over all the nums to count how many are smaller
        let mut result: Vec<i32> = vec![0; nums.len()]; 

        for (i, num) in nums.iter().enumerate() {
            for num2 in &nums {
                if *num2 < *num {
                    result[i] += 1;
                }
            }
        }

        result
    }

    pub fn smaller_numbers_than_current_2(nums: Vec<i32>) -> Vec<i32> {
        println!("{:?}", nums);

        // Create sorted vector
        let mut sorted_nums: Vec<i32> = nums.clone();
        sorted_nums.sort();

        // Create hashmap from sorted vector which will give us the result
        use std::collections::HashMap;

        let mut nums_map: HashMap<i32, usize> = HashMap::new();
        for (index, val) in sorted_nums.iter().enumerate() {
            nums_map.entry(*val).or_insert(index);   
        }

        // Create result vector and use hash map to populate
        let mut result: Vec<i32> = Vec::new();
        for num in nums.iter() {
            result.push(*nums_map.get(num).unwrap() as i32);
        }

        result
    }
}

fn main() {
    // println!("{:?}", Solution::smaller_numbers_than_current(vec![8,1,2,2,3]));
    println!("{:?}", Solution::smaller_numbers_than_current_2(vec![8,1,2,2,3]));
}
