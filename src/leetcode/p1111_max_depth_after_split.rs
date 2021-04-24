struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut current: i32 = 0;
        let mut ret: Vec<i32> = Vec::new();
        seq.chars().for_each(|ch| {
            match ch {
                '(' =>{
                    current = 1 - current;
                    ret.push(current);
                }
                ')' =>{
                    ret.push(current);
                    current = 1 - current;
                }
                _ => {}
            }
        });
        ret
    }
}