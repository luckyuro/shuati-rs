use super::Solution;
use std::collections::BTreeSet;
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut ans_set = BTreeSet::<&str>::new();
        for x in s.chars().into_iter().enumerate() {
            println!("{:?}", x);
        }
        5
    }

    fn check(s: &str, ans_set: &mut BTreeSet<&str>) {}
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_unique_split() {
        assert_eq!(Solution::max_unique_split("ababccc".to_string()), 5);
    }
}
