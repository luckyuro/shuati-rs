//6kyu
//kata_URL:
//          https://www.codewars.com/kata/consecutive-strings/train/rust

#[allow(dead_code)]
pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let l = strarr.len();
    if l == 0 || k > l || k <= 0 {
        return "".to_string();
    }

    let mut max = 0;
    for i in 0..k {
        max += strarr[i].len();
    }

    let mut sum = max;
    let mut max_i = 0;

    for i in 1..l-k+1 {
        sum = sum - strarr[i-1].len() + strarr[i+k-1].len();
        if sum > max {
            max = sum;
            max_i = i;
        }
    }

    let mut ret = "".to_string();
    for i in 0..k{
        ret += &strarr[max_i+i].to_string();
    }

    ret

}


//fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
//    let mut result = String::new();
//
//    if k > 0 && strarr.len() >= k {
//        for index in 0..strarr.len() - k + 1 {
//            let s: String = strarr[index..index + k].join("");
//            if s.len() > result.len() {
//                result = s;
//            }
//        }
//    }
//
//    result
//}

#[cfg(test)]
mod test {
    use super::longest_consec;

    fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
        assert_eq!(&longest_consec(strarr, k), exp)
    }

    #[test]
    fn basics_longest_consec() {
        testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
        testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1,
                "oocccffuucccjjjkkkjyyyeehh");
        testing(vec![], 3, "");
        testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
        testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
        testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
    }

}