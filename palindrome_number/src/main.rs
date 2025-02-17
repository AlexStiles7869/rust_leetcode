struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // Convert number to string and then compare to reversed version
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }
}

fn main() {
    println!("{:?}", Solution::is_palindrome(-121));
}