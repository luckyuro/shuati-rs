//6kyu
//kata_URL:
//          https://www.codewars.com/kata/5592e3bd57b64d00f3000047/train/rust

#[allow(dead_code)]
pub fn find_nb(n: u64) -> i32 {
    let inner = n*4;
    let bot = (inner as f64).sqrt().sqrt() as u64;

    if (bot*(bot+1)) as f64 == (inner as f64).sqrt(){
        bot as i32
    } else {
        -1
    }
}

//fn find_nb(n: u64) -> i32 {
//    let kk = (4.0 * n as f64).sqrt().sqrt().floor() as u64;
//    if 4u64 * n == kk * kk * (kk + 1u64) * (kk + 1u64)
//        {kk as i32}
//        else
//        {-1}
//}

#[cfg(test)]
mod test {
    use super::find_nb;

    fn testing(n: u64, exp: i32) -> () {
        assert_eq!(find_nb(n), exp);
    }

    #[test]
    fn basics_find_nb() {
        testing(4183059834009, 2022);
        testing(24723578342962, -1);
        testing(135440716410000, 4824);
        testing(40539911473216, 3568);
        testing(26825883955641, 3218);
        testing(1025292944081385001, 45001);
    }

}