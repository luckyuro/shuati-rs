use super::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // let mut all: Vec<Vec<String>> = Vec::new();
        // all.push(vec!["".to_string()]);
        // all.push(vec!["()".to_string()]);
        // for i in 2..=n as usize {
        //     let mut result = Vec::new();
        //     for x in 0..i {
        //         for left in all[x].iter() {
        //             for right in all[i - 1 - x].iter() {
        //                 result.push(format!("({}){}", left, right));
        //             }
        //         }
        //     }
        //     all.push(result);
        // }
        // all.pop().unwrap()

        fn sub_cur(s: String, n: usize, left: usize, right: usize, result: &mut Vec<String>) {
            if left == right && left + right == 2 * n {
                result.push(s);
                return;
            }
            if left < n {
                sub_cur(format!("{}(", &s), n, left + 1, right, result);
            }
            if right < left {
                sub_cur(format!("{})", &s), n, left, right + 1, result);
            }
        }
        let mut ret = Vec::new();
        sub_cur("".to_string(), n as usize, 0, 0, &mut ret);
        ret
    }
}
