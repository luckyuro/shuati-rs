//7kyu
//kata_URL:
//          https://www.codewars.com/kata/56dbe0e313c2f63be4000b25/train/rust

pub fn vert_mirror(s: String) -> String {
    let d: Vec<&str> = s.split('\n').collect();
    let nd: Vec<String> = d
        .into_iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect();
    nd.join("\n")
}
pub fn hor_mirror(s: String) -> String {
    let mut d: Vec<&str> = s.split('\n').collect();
    d.reverse();
    let nd: Vec<String> = d
        .into_iter()
        .map(|x| String::from(x))
        .collect();
    nd.join("\n")
}

pub fn oper(func: fn(String) -> String, s: String) -> String {
    func(s)
}


//fn hor_mirror(s: String) -> String {
//    s.split('\n').rev().collect::<Vec<&str>>().join("\n")
//}
//fn vert_mirror(s: String) -> String {
//    s.split('\n').map(|s| s.chars().rev().collect::<String>()).collect::<Vec<String>>().join("\n")
//}
//fn oper(oper: fn(String) -> String, s: String) -> String {
//    oper(s)
//}

//fn oper<F: FnOnce(String) -> String>(f: F, s: String) -> String {
//    // your code
//    f(s)
//}

#[cfg(test)]
mod test {
    use super::{hor_mirror,oper, vert_mirror};
    fn testing1(s: &str, exp: &str) -> () {
        assert_eq!(oper(hor_mirror, s.to_string()), exp)
    }
    fn testing2(s: &str, exp: &str) -> () {
        assert_eq!(oper(vert_mirror, s.to_string()), exp)
    }

    #[test]
    fn basics_oper() {

        testing1("lVHt\nJVhv\nCSbg\nyeCt", "yeCt\nCSbg\nJVhv\nlVHt");
        testing1("njMK\ndbrZ\nLPKo\ncEYz", "cEYz\nLPKo\ndbrZ\nnjMK");
        testing1("QMxo\ntmFe\nWLUG\nowoq", "owoq\nWLUG\ntmFe\nQMxo");

        testing2("hSgdHQ\nHnDMao\nClNNxX\niRvxxH\nbqTVvA\nwvSyRu", "QHdgSh\noaMDnH\nXxNNlC\nHxxvRi\nAvVTqb\nuRySvw");
        testing2("IzOTWE\nkkbeCM\nWuzZxM\nvDddJw\njiJyHF\nPVHfSx", "EWTOzI\nMCebkk\nMxZzuW\nwJddDv\nFHyJij\nxSfHVP");
        testing2("cuQW\nxOuD\nfZwp\neqFx", "WQuc\nDuOx\npwZf\nxFqe");

    }
}