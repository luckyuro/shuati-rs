//6kyu
//kata_URL:
//          https://www.codewars.com/kata/544aed4c4a30184e960010f4/train/rust

#[allow(dead_code)]
pub fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut ret:Vec<u32> = vec![];
    for i in 2u32..integer/2+1{
        if integer%i == 0 {
            ret.push(i)
        }
    }
    if ret.len() == 0{
        Err(format!("{} is prime", integer))
    } else {
        Ok(ret)
    }
}

//#[test]
//fn tests() {
//    assert_eq!(divisors(15), Ok(vec![3,5]));
//    assert_eq!(divisors(12), Ok(vec![2,3,4,6]));
//    assert_eq!(divisors(13), Err("13 is prime".to_string()));
//}

// Others' solution good ones
//fn divisors(integer: u32) -> Result<Vec<u32>, String> {
//    let divs = (2..integer)
//        .filter(|k| integer % k == 0)
//        .collect::<Vec<u32>>();
//
//    if divs.len() > 0 {
//        Ok(divs)
//    } else {
//        Err(format!("{} is prime", integer))
//    }
//}

//let rt = (integer as f64).sqrt() as u32 + 1;


#[cfg(test)]
mod test {
    use super::divisors;
    #[test]
    fn correct_divisor() {
        assert_eq!(divisors(15), Ok(vec![3,5]));
        assert_eq!(divisors(12), Ok(vec![2,3,4,6]));
        assert_eq!(divisors(13), Err("13 is prime".to_string()));
    }
}