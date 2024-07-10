use super::Solution;

impl Solution {
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        let mut ans = 0i32;
        let mut last = 0i32;
        for r in rungs.into_iter() {
            ans += (r - last - 1) / dist;
            last = r;
        }
        ans
    }
}
