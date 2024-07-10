use super::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        fn common(s1: &str, s2: &str) -> usize {
            let short = std::cmp::min(s1.len(), s2.len());
            let mut end = 0usize;
            let (mut chars1, mut chars2) = (s1.chars(), s2.chars());
            while end < short && chars1.next() == chars2.next() {
                end += 1;
            }
            end
        }
        let mut prefix = strs[0].as_str();
        for i in 1..strs.len() {
            prefix = prefix.split_at(common(prefix, strs[i].as_str())).0;
            if prefix.is_empty() {
                break;
            }
        }
        prefix.to_string()
    }
}

#[cfg(test)]
mod test {

    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ]),
            "fl".to_string()
        );
    }
}
