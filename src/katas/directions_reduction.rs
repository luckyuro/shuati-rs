//5kyu
//kata_URL:
//          https://www.codewars.com/kata/directions-reduction/train/rust

pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

use self::Direction::*;

pub fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut ret: Vec<Direction> = Vec::new();

    for ele in arr.iter() {
        match ele {
            &NORTH => match ret.last() {
                Some(&SOUTH) => {
                    ret.pop();
                }
                _ => ret.push(NORTH),
            },
            &SOUTH => match ret.last() {
                Some(&NORTH) => {
                    ret.pop();
                }
                _ => ret.push(SOUTH),
            },
            &EAST => match ret.last() {
                Some(&WEST) => {
                    ret.pop();
                }
                _ => ret.push(EAST),
            },
            &WEST => match ret.last() {
                Some(&EAST) => {
                    ret.pop();
                }
                _ => ret.push(WEST),
            },
            // _ => {}
        }
    }
    ret
}

//fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
//    use Direction::*;
//    let mut ret: Vec<Direction> = Vec::new();
//
//    for ele in arr.iter() {
//        match ele {
//            &NORTH =>
//                match ret.last() {
//                    Some(&SOUTH) => { ret.pop(); },
//                    _ => ret.push(NORTH),
//                },
//            &SOUTH =>
//                match ret.last() {
//                    Some(&NORTH) => { ret.pop(); },
//                    _ => ret.push(SOUTH),
//                },
//            &EAST =>
//                match ret.last() {
//                    Some(&WEST) => { ret.pop(); },
//                    _ => ret.push(EAST),
//                },
//            &WEST =>
//                match ret.last() {
//                    Some(&EAST) => { ret.pop(); },
//                    _ => ret.push(WEST),
//                },
//        }
//    }
//    ret
//}

//fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
//    let mut result: Vec<Direction> = Vec::new();
//    for &s in arr {
//        if !result.is_empty() && can_be_reduced(s, *result.last().unwrap()) {
//            result.pop();
//        } else {
//            result.push(s);
//        }
//    }
//    result
//}
//
//fn can_be_reduced(elem: Direction, last: Direction) -> bool {
//    use Direction::*;
//    match (elem, last) {
//        (NORTH, SOUTH) | (SOUTH, NORTH) | (WEST, EAST) | (EAST, WEST) => true,
//        _ => false,
//    }
//}

//use Direction::*;
//
//impl Direction {
//    fn opposite(&self, other: &Direction) -> bool {
//        match (self, other) {
//            (&NORTH, &SOUTH) | (&SOUTH, &NORTH) => true,
//            (&WEST, &EAST) | (&EAST, &WEST) => true,
//            _ => false,
//        }
//    }
//}
//
//fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
//    let arr = arr.to_vec();
//    reduc(arr)
//}
//
//fn reduc(arr: Vec<Direction>) -> Vec<Direction> {
//    for i in 1..arr.len() {
//        if arr[i - 1].opposite(&arr[i]) {
//            let mut arr = arr.clone();
//            arr.remove(i - 1);
//            arr.remove(i - 1);
//            return reduc(arr);
//        }
//    }
//    arr
//}

#[cfg(test)]
mod test {}
