use num::traits::Num;

struct Fenwick<T: Num> {
    data: Vec<T>,
    max_index: usize,
}

impl <T> Fenwick<T> where {
    fn construct(size: usize) -> Fenwick<T> {
        let vec: Vec<T> = Vec::with_capacity(size);
        Fenwick { data: vec, max_index: size-1}
    }

    fn update(&mut self, pos: &usize, delta: &T) {
        let &mut i= pos.clone();
        while i <= self.max_index {
            self.data[i] += delta;
            i |= i+1;
        }
    }

    fn set(&mut self, pos: &usize, val: &T) {
        self.update(&pos, val-self.data[&pos])
    }

    fn prefix_sum(&self, pos: &usize) -> Option<&T> {
        let &mut i = pos.clone();
        let sum: T = 0;
        while i > 0 {
            sum += self.data[i];
            i = (i & (i+1)) - 1;
        }
        return sum;
    }

//    fn range_sum(&self, start: usize, end: usize) -> Option<&T> {
//
//    }

//    fn scale(&mut self, cor: T) {
//        for x in self.data.iter_mut() {
//            x *= cor;
//        }
//    }


}


struct FenwickOneBased<T> {
    data: Vec<T>,
    max_index: usize,
}

impl <T> FenwickOneBased<T> {
    fn construct(size: usize) -> Fenwick<T> {
        let vec: Vec<T> = Vec::with_capacity(size + 1);
        Fenwick { data: vec, max_index: size}
    }

    fn update(&mut self, pos: &usize, delta: &T) {
        let &mut i= pos.clone();
        while i <= self.data.len() {
            self.data[i] += delta;
            i += i & -i;
        }
    }

    fn set(&mut self, pos: &usize, val: &T) {
        self.update(&pos, val-self.data[&pos])
    }

    fn prefix_sum(&self, pos: &usize) -> Option<&T> {
        let &mut i = pos.clone();
        let sum: T = 0;
        while i > 0 {
            sum += self.data[i];
            i -= i & -i;
        }
        return Some(&sum);
    }

//    fn range_sum(&self, start: usize, end: usize) -> Option<&T> {
//
//    }

//    fn scale(&mut self, cor: T) {
//        for x in self.data.iter_mut() {
//            x *= cor;
//        }
//    }

}


#[cfg(test)]
mod test {
    use super::Fenwick;
    use super::FenwickOneBased;

    #[test]
    fn test_find_kth() {
    }

    #[test]
    fn test_find_median() {
    }
}