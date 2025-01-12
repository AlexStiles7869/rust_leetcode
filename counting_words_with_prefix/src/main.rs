struct Solution {}

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut count: i32 = 0;
        for word in words.iter() {
            if word.starts_with(&pref) {
                count += 1
            }
        }
        count
    }
}

fn main() {
    println!("{}", Solution::prefix_count(vec!["pay".to_string(), "attention".to_string(), "practice".to_string(), "attend".to_string()], "at".to_string()));
}

