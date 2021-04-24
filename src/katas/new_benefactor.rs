//7kyu
//kata_URL:
//          https://www.codewars.com/kata/569b5cec755dd3534d00000f/train/rust

#[allow(dead_code)]
pub fn new_avg(arr: &[f64], newavg: f64) -> Option<i32> {
    let mut ans:f64 = newavg;

    for i in arr.iter(){
        ans += newavg - i;
    }

    if ans <= 0.0 {
        return None;
    } else {
        return Some(ans.ceil() as i32);
    }
}

//fn new_avg(arr: &[f64], newavg: f64) -> Option<i32> {
//    match (arr.len() as f64 + 1f64) * newavg - arr.iter().sum::<f64>() {
//        n if n > 0f64 => Some(n.ceil() as i32),
//        _ => None
//    }
//}

#[cfg(test)]
mod test {
    use super::new_avg;

    fn testing(arr: &[f64], newavg: f64, exp: Option<i32>) -> () {
        assert_eq!(exp, new_avg(arr, newavg))
    }

    #[test]
    fn new_benefactor_tests() {
        let a1 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 16.0];
        testing(&a1, 90.0, Some(628));
        let a2 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
        testing(&a2, 92.0, Some(645));
        let a3 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
        testing(&a3, 2.0, None);
        let a4 = [14000.25, 300.76, 50.56, 70.0, 90.0, 11.0, 150.48, 1200.98];
        testing(&a4, 4800.0, Some(27326));
    }
}