use super::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0i32;
        let mut last = 'M';
        for c in s.chars() {
            match c {
                'I' => ans += 1,
                'V' => ans += 5,
                'X' => ans += 10,
                'L' => ans += 50,
                'C' => ans += 100,
                'D' => ans += 500,
                'M' => ans += 1000,
                _ => continue,
            }
            match last {
                'I' => {
                    if c != 'I' {
                        ans -= 2;
                    }
                }
                'X' => {
                    if c == 'L' || c == 'C' {
                        ans -= 20;
                    }
                }
                'C' => {
                    if c == 'D' || c == 'M' {
                        ans -= 200;
                    }
                }
                _ => (),
            }
            last = c;
        }
        ans
    }
}
