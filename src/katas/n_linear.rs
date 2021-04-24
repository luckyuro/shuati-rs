//4kyu
//kata_URL:
//          https://www.codewars.com/kata/n-linear/train/rust

fn n_linear(m: &[u32], n: usize) -> u32 {

    let mut mptr_vec = Vec::with_capacity(m.len());
    let mut mvalue_vec = Vec::with_capacity(m.len());
    let mut ans_vec = Vec::with_capacity(n+1);

    for i in 0..m.len() {
        mptr_vec.push(0);
        mvalue_vec.push(m[i] * 1 + 1)
    }
    for i in 0..=n{
        ans_vec.push(1u32);
    }

    for l in 0..n {
        let mut min = std::u32::MAX;
        for i in 0..m.len() {
            if mvalue_vec[i] < min { min = mvalue_vec[i]; }
        }

        ans_vec[l+1] = min;

        for i in 0..m.len() {
            if mvalue_vec[i] == min {
                mptr_vec[i] += 1;
                mvalue_vec[i] = ans_vec[mptr_vec[i]] * m[i] + 1;
            }
        }
    }

    ans_vec[n]

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_test() {
        assert_eq!(n_linear(&[2, 3], 10), 22);
        assert_eq!(n_linear(&[3, 2], 10), 22);
    }

    #[test]
    fn triplet_test() {
        assert_eq!(n_linear(&[5, 7, 8], 10), 64);
        assert_eq!(n_linear(&[5, 7, 8], 11), 65);
    }
}


//
//fn n_linear(m: &[u32], n: usize) -> u32 {
//    use std::collections::{BTreeMap, HashMap};
//
//    let mut v2coff = BTreeMap::new();
//    let mut coff_seq = HashMap::with_capacity(m.len());
//    for co in m {
//        coff_seq.insert(*co, Vec::with_capacity(n));
//    }
//    v2coff.insert(1u32, m.to_vec());
//
//
//    let mut count = 0;
//    loop {
//        // Get the smallest number for BtreeMap
//        // coffs store all the coefficients that generated this value, the coffs should move on
//        let (&ans, _coffs_gen_ans) = v2coff.range(..).next().unwrap();
//        let coffs_gen_ans = _coffs_gen_ans.clone();
//        // if the n is enough, just return anf finish
//        if count == n {
//            return ans
//        }
//
//        // insert the newly generated to the list of all coffs
//        for c in m {
//            let v = coff_seq.get_mut(c).unwrap();
//            v.push(ans);
//
//            // if the coff is in the list that generated the value
//            if coffs_gen_ans.contains(c) {
//                let value = v[0] * (*c) + 1;
//                v.remove(0);
//
//                // push to the tail of add to BtreeMap
//                if v2coff.contains_key(&value) {
//                    v2coff.get_mut(&value).unwrap().push(*c);
//                } else {
//                    v2coff.insert(value, vec![*c]);
//                }
//            }
//        }
//        v2coff.remove(&ans);
//        count += 1;
//    }
//}


//fn n_linear(m: &[u32], n: usize) -> u32 {
//    use std::collections::HashMap;
//    let mut map = HashMap::with_capacity(m.len());
//    for co in m {
//        map.insert(*co, Vec::with_capacity(n) );
//    }
//
//    let mut ans = 1;
//    let mut count = 0;
//
//    let mut blacked = vec![];
//
//    loop {
//        if count == n {
//            return ans
//        }
//
//        let mut min = std::u32::MAX;
//        for co in m {
//            let v = map.get_mut(co).unwrap();
//
//            if v.len() > (n - count + 1) {
//                if map.get(co).unwrap()[0] < min {
//                    min = map.get(co).unwrap()[0];
//                }
//
//                continue
//            } else if blacked.contains(co) {
//                continue
//            }
//            let (_t, _) = (*co).overflowing_mul(ans);
//            let (_t, flag) = _t.overflowing_add(1);
//            if flag {blacked.push(*co)}
//            map.get_mut(co).unwrap().push(_t);
//
//
//            if map.get(co).unwrap()[0] < min {
//                min = map.get(co).unwrap()[0];
//            }
//        }
//
//        for co in m {
//            let v = map.get_mut(co).unwrap();
//            if v[0] == min { v.remove(0); }
//        }
//
//        ans = min;
//        count += 1;
//
//    }
//
//}

#[cfg(test)]
mod test {

}