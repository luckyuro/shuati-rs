use super::Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut x = 0u32;
        for op in logs.iter() {
            match op.as_str() {
                "./" => continue,
                "../" => x = x.saturating_sub(1),
                _ => x += 1,
            }
        }
        x as i32
    }
}
