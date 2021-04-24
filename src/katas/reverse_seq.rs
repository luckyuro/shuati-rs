//8kyu
//kata_URL:
//          http://www.codewars.com/kata/reversed-sequence/train/rust

fn reverse_seq(n: u32) -> Vec<u32> {
    (1u32..n+1).rev().collect()
}

#[cfg(test)]
mod test {

}