use parser;

pub fn main() {
    let c = parser::Content::read_file(&"input.txt").expect("No such file found");
    let res = day04::pairs_vec(&c);
    let fco: u32 = res.iter().map(|x| if x.fully_contains_other() { 1 } else { 0 }).sum();
    println!("fully_contains: {}", fco);
    let co: u32 = res.iter().map(|x| if x.contains_other() { 1 } else { 0 }).sum();
    println!("contains: {}", co);
}

#[cfg(test)]
mod tests {
    use day04;
    use parser;

    #[test]
    fn t1_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(!res[0].fully_contains_other());
    }

    #[test]
    fn t2_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(!res[1].fully_contains_other());
    }
    #[test]
    fn t3_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(res[3].fully_contains_other());
    }
    #[test]
    fn t4_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(res[4].fully_contains_other());
    }
    #[test]
    fn t5_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(!res[5].fully_contains_other());
    }
    #[test]
    fn t6_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(!res[0].contains_other());
    }

    #[test]
    fn t7_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(!res[1].contains_other());
    }
    #[test]
    fn t8_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(res[2].contains_other());
    }
    #[test]
    fn t9_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(res[3].contains_other());
    }
    #[test]
    fn t10_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(res[4].contains_other());
    }
    fn t11_test() {
        let input = parser::Content::read(&"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        let res = day04::pairs_vec(&input);
        assert!(res[5].contains_other());
    }
}
