struct Solution {}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut total_time: i32 = 0;

        let mut current_location: Vec<i32> = points[0].clone();
        for point in &points[1..] {
            while current_location[0] != point[0] || current_location[1] != point[1] {
                if (point[0] - current_location[0]).abs() >= 1 {
                    current_location[0] += if point[0] > current_location[0] { 1 } else { -1 }
                }

                if (point[1] - current_location[1]).abs() >= 1 {
                    current_location[1] += if point[1] > current_location[1] { 1 } else { -1 }
                }

                // Increment the total time 
                total_time += 1; 
            }
        }

        total_time
    }
}


fn main() {
    println!("{}", Solution::min_time_to_visit_all_points(vec![vec![1,1], vec![3,4], vec![-1,0]]));
    println!("Seperator");
    println!("{}", Solution::min_time_to_visit_all_points(vec![vec![3,2], vec![-2, 2]]));
}
