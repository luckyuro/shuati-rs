use super::Solution;
impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut flag = 0;
        let new_str = str.trim();
        let mut abs_num = String::new();

        for c in new_str.chars() {
            if c.is_digit(10) {
                flag = if flag==0 { 1 } else { flag };
                abs_num.push(c);
            } else if flag != 0 {
                break;
            } else if c == '-' {
                flag = -1;
            } else if c == '+' {
                flag = 1;
            } else {
                break;
            }
        }

        if abs_num.len() > 0{
            if flag == -1 {
                let num = abs_num.parse::<i32>().unwrap_or(std::i32::MIN);
                if num < 0 {
                    num
                } else {
                    -1 * num
                }
            } else {
                abs_num.parse::<i32>().unwrap_or(std::i32::MAX)
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_my_atoi() {
        assert_eq!(Solution::my_atoi(String::from("42")), 42);
        assert_eq!(Solution::my_atoi(String::from(" -42")), -42);
        assert_eq!(Solution::my_atoi(String::from("wwww")), 0);
        assert_eq!(Solution::my_atoi(String::from("42w")), 42);
        assert_eq!(Solution::my_atoi(String::from("+1")), 1);
        assert_eq!(Solution::my_atoi(String::from("-91283472332")), -2147483648);
    }
}