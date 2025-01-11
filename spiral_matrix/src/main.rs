// This is frankly pretty disgusting but it gets the job done...
struct Solution {}

impl Solution {
    // Create spiralised matrix. The matrix is assumed to not be empty.
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m: usize = matrix.len();
        let n: usize = (matrix[0]).len();
        let num_elements: usize = m * n;

        // Spiralise the matrix
        let mut spiral_matrix: Vec<i32> = Vec::new();
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut offset: usize = 0;

        while spiral_matrix.len() != num_elements {

            // Push the correct element on to the result vector
            spiral_matrix.push(matrix[i][j]);

            // i -> m | j -> n & i -> vert | j -> horz
            if i == offset {
                if j < (n - offset - 1) {
                    j += 1;
                } else {
                    i += 1;
                }
            } else if i < (m - offset - 1) { 
                if j == (n - offset - 1) { 
                    i += 1; 
                } else if j <= (offset + 1) {
                    if i == (offset + 1) {
                        offset += 1;
                        j += 1;
                    } else {
                        i -= 1;
                    }
                } else {
                    j += 1;
                }
            } else {
                if j > offset {
                    j -= 1;
                    // if j == (offset + 1) {
                        // offset += 1
                    // }
                } else {
                    i -= 1;
                }
            }
        }

        spiral_matrix
    }
}
fn main() {
    println!("{:?}", Solution::spiral_order(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12],vec![13,14,15,16],vec![17,18,19,20],vec![21,22,23,24]]));
}
