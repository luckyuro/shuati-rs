//5kyu
//kata_URL:
//          https://www.codewars.com/kata/is-my-friend-cheating/train/rust

pub  fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    // your code
    let m = m as usize;
    let sum = (1 + m ) * m / 2;
    let mut ans = vec![];

    for x in sum/m..m {
        let y = (sum - x ) / (1 + x);
        if x * y == sum - x - y {
            ans.push((x as i32, y as i32))
        }
    }

//    {
//        let mut ret = vec![];
//
//        for (x, y) in ans.iter() {
//            ret.push((*x, *y))
//        }
//
//        for (x, y) in ans.iter().rev() {
//            ret.push((*y, *x))
//        }
//
//        ret
//    }
    ans
}

#[cfg(test)]
mod test {
    use super::remove_nb;

    fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
        assert_eq!(remove_nb(n), exp)
    }

    #[test]
    fn basics_remove_nb() {

        testing(26, vec![(15, 21), (21, 15)]);
        testing(100, vec![]);
        testing(101, vec![(55, 91), (91, 55)]);
        testing(102, vec![(70, 73), (73, 70)]);

    }
}