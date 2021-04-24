//6kyu
//kata_URL:
//          https://www.codewars.com/kata/round-by-0-dot-5-steps/rust

#[allow(dead_code)]
pub fn solution(n: f64) -> f64 {
    let mid = (n as i32) as f64 + 0.5;
    let diff = n - mid;
    if diff < -0.25 {
        return mid - 0.5;
    } else if diff > 0.25 {
        return mid + 0.5;
    } else {
        return mid;
    }
}

// (2.0 * n).round() / 2.0

//fn solution(n: f64) -> f64 {
//    if n % 1.0 > 0.75 { n.floor() + 1.0 }
//        else if n % 1.0 > 0.25 {n.floor() + 0.5 }
//            else { n.floor() }
//}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn half_step_tests() {
        assert_eq!(solution(4.2), 4.0);
        assert_eq!(solution(4.4), 4.5);
        assert_eq!(solution(4.6), 4.5);
        assert_eq!(solution(4.8), 5.0);
    }
}