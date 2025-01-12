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

    pub fn spiral_order_enum(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        enum Direction {
            Up,
            Down,
            Right,
            Left
        }

        let mut direction = Direction::Right;
        let mut round = 0;
        let mut row = 0;
        let mut col = 0;
        let m = matrix.len();
        let n = matrix[0].len();
        let mut spiral_matrix = vec![];

        for _ in 0..(m * n) {
            spiral_matrix.push(matrix[row][col]);

            match direction {
                Direction::Up => {
                    if row - 1 == round {
                        col += 1;
                        direction = Direction::Right; 
                        round += 1;
                    } else {
                        row -= 1;
                    }
                }
                Direction::Down => {
                    if row + 1 == m - round {
                        col -= 1;
                        direction = Direction::Left;
                    } else {
                        row += 1;
                    }
                }
                Direction::Right => {
                    if col + 1 == n - round {
                        row += 1;
                        direction = Direction::Down;
                    } else {
                        col += 1;
                    }
                }
                Direction::Left => {
                    if col == round {
                        row -= 1;
                        direction = Direction::Up;
                    } else {
                        col -= 1;
                    }
                }
            }
        }

        spiral_matrix
    }
}
fn main() {
    println!("{:?}", Solution::spiral_order(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12],vec![13,14,15,16],vec![17,18,19,20],vec![21,22,23,24]]));
    println!("{:?}", Solution::spiral_order_enum(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12],vec![13,14,15,16],vec![17,18,19,20],vec![21,22,23,24]]));
}
