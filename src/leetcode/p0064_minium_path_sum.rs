use super::Solution;
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let lines = grid.len();
        let cols = grid[0].len();
        let mut path = grid;

        for x in 0..lines {
            for y in 0..cols {
                if x == 0 && y == 0 {
                    ()
                } else if x == 0 {
                    path[0][y] += path[0][y - 1];
                } else if y == 0 {
                    path[x][0] += path[x - 1][0];
                } else {
                    path[x][y] += std::cmp::min(path[x - 1][y], path[x][y - 1])
                }
            }
        }
        path[lines - 1][cols - 1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case1() {
        let case = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(7, Solution::min_path_sum(case));
    }
}
