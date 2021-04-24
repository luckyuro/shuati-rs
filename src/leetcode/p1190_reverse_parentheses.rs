struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut pair: HashMap<usize, usize> = HashMap::new();
        let mut opened_parentheses: Vec<usize> = Vec::new();
        let s_bytes = s.as_bytes();

        for (p, c) in s_bytes.iter().enumerate() {
            if *c == '(' as u8 {
                opened_parentheses.push(p)
            } else if *c == ')' as u8{
                let o = opened_parentheses.pop().unwrap();
                pair.insert(o, p);
                pair.insert(p, o);
            }
        }
        let mut buffer: Vec<u8> = Vec::with_capacity(s.len());
        let mut i: usize = 0;
        let mut forward: bool = true;

        while i < s.len() {
            if s_bytes[i] == '(' as u8 || s_bytes[i] == ')' as u8{
                i = *pair.get(&i).unwrap();
                forward = !forward;
            } else {
                buffer.push(s_bytes[i]);
            }
            i = if forward { i + 1 } else { i - 1 }
        }

        return  String::from_utf8(buffer).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_is_match() {
        assert_eq!(Solution::reverse_parentheses(String::from("(ed(et(oc))el)")),
        String::from("leetcode"));
    }
}