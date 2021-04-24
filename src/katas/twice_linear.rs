//4kyu
//kata_URL:
//          https://www.codewars.com/kata/twice-linear/train/rust

pub fn dbl_linear(n: u32) -> u32{
    // your code
    use std::collections::VecDeque;
    let mut q2 = VecDeque::with_capacity(n as usize);
    let mut q3 = VecDeque::with_capacity(n as usize);

    let mut count = 0u32;
    let mut ans = 1u32;
    q2.push_back(3u32);
    q3.push_back(4u32);
    loop {
        if count == n {
            return ans;
        } else {
            if q2[0] <= q3[0] {
                ans  = q2.pop_front().unwrap();
                count += 1;

                if &ans == q3.get(0).unwrap() {
                    q3.pop_front();
                }
            } else {
                ans  = q3.pop_front().unwrap();
                count += 1;
            }


            q2.push_back(2*ans + 1);
            q3.push_back(3*ans + 1);
        }
    }


//    use std::collections::BTreeSet;
//    let mut set = BTreeSet::new();
//    set.insert(1u32);
//
//    let mut count = 0;
//    loop {
//        let num = *(set.range(..).next().unwrap());
//        set.remove(&num);
//        if count == n {
//            return num
//        } else {
//            set.insert(2*num + 1);
//            set.insert(3*num + 1);
//            count += 1;
//        }
//
//    }

//    heap not work should unique
//    let mut heap = BinaryHeap::new();
//    heap.push(Reverse(1u32));

//    let mut count = 0;
//    loop {
//        let num = match heap.pop() {
//            Some(Reverse(x)) => x,
//            None => panic!()
//        };
//        if count  == n {
//            return num
//        } else {
//            heap.push(Reverse(2*num + 1));
//            heap.push(Reverse(3*num + 1));
//            count += 1;
//        }
//    }
}

#[cfg(test)]
mod tests {
    use super::dbl_linear;
    fn testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }

    #[test]
    fn basics_dbl_linear() {
        testing(10, 22);
        testing(20, 57);
        testing(30, 91);
        testing(50, 175);
        testing(100, 447);
    }
}