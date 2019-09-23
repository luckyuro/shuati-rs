struct Solution;

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut name_p: usize = 0;
        let name_bytes = name.as_bytes();

        for ch in typed.as_bytes().iter() {
            if name_p < name.len() && name_bytes[name_p] == *ch {
                name_p += 1;
                continue;
            } else if name_p > 0 && name_bytes[name_p - 1] == *ch {
                continue;
            } else {
                return false;
            }
        }
        name_p >= name.len()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_long_pressed_name(String::from("leelee"), String::from("lleeelee")),
                   true);
        assert_eq!(Solution::is_long_pressed_name(String::from("kikcxmvzi"), String::from("kiikcxxmmvvzz")),
                   false);
    }
}