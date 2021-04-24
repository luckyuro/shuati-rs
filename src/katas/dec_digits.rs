//7kyu
//kata_URL:
//          https://www.codewars.com/kata/number-of-decimal-digits/train/rust

#[allow(dead_code)]
pub fn digits(n: u64) -> usize{
    let mut d:usize = 1;
    let mut mn:u64 = n;
    while  mn/10 != 0 {
        mn = mn/10;
        d += 1;
    }
    d
}

//n.to_string().len()


//let mut l = 1;
//while n >= 10 {
//n /= 10;
//l += 1;
//}
//l

#[cfg(test)]
mod test {
    use super::digits;

    #[test]
    fn digits_test() {
        assert_eq!(digits(5), 1);
        assert_eq!(digits(12345), 5);
        assert_eq!(digits(9876543210), 10);
    }
}