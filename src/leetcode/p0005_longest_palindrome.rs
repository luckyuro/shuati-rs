extern crate num;
pub fn longest_palindrome(s: String) -> String {
    let l: usize = s.len();
    let sc = s.as_bytes();
    let mut state = [[0usize; 1000]; 1000];
    let mut max: usize = 0;
    let mut start: usize = 0;
    let mut end: usize = 0;

    if l < 2 {
        return s;
    }

//    for i in num::iter::range(0, l).rev() {
    for i in (0..l).rev(){
//        for j in num::iter::range(i, l) {
        for j in i..l{
            if sc[i] == sc[j] {
                let c = if i == j {
                    1
                } else if i + 1 == j {
                    2
                } else if state[i+1][j-1] > 0 {
                    state[i+1][j-1] + 2
                } else {
                    0
                };

                if c > max {
                    max = c;
                    start = i;
                    end = j;
                }
                state[i][j] = c;

            } else {
                state[i][j] = 0
            }
        }
    }

    String::from(&s[start..=end])
}
