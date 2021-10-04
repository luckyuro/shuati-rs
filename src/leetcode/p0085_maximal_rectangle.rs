use super::Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let x = matrix.len();
        let y = matrix[0].len();
        // max '1' for row, max '1' for column
        let mut helper = vec![vec![(0, 0, 0); y]; x];
        let mut max = 1;
        for (x, vec) in matrix.iter().enumerate() {
            for (y, char) in vec.iter().enumerate() {
                match char {
                    '0' => {
                        helper[x][y] = (0, 0, 0)
                    }
                    '1' => {
                        let x_max = if y > 0 { helper[x][y-1].0 } else { 0 };
                        let y_max = if x > 0 { helper[x-1][y].1 } else { 0 };

                        let _max = std::cmp::max(x_max, y_max);
                        max = std::cmp::max(_max, max);

                        if x > 0 && y > 0 {
                            let (_x, _y, _r) = helper[x-1][y-1];

                            helper[x][y] = (x_max + 1, y_max + 1, _r +1 );
                        } else {
                            helper[x][y] = (x_max + 1, y_max + 1, 1);
                        }
                    }
                    _ => {}
                }

            }
        }
        max
    }
}