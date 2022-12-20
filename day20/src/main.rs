use parser;

const PATH: &str = "data";
const FILENAME: &str = "day20";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let list = day20::create_list(&c);
    let res_a = day20::execute(&list, &vec![1000, 2000, 3000], 1, 1);
    let res_b = day20::execute(&list, &vec![1000, 2000, 3000], 10, 811589153);
    //println!("Part A: {}", res_a);
    //println!("Part B: {}", res_b);
}

#[cfg(test)]
mod tests {
    use day20;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"1\n2\n-3\n3\n-2\n0\n4");
        let list = day20::create_list(&c);
        assert_eq!(day20::execute(&list, &vec![1000, 2000, 3000], 1, 1), 3)
    }

    #[test]
    fn t2_test() {
        let c = parser::Content::read(&"1\n2\n-3\n3\n-2\n0\n4");
        let list = day20::create_list(&c);
        assert_eq!(day20::execute(&list, &vec![1000, 2000, 3000], 10, 811589153), 1623178306)
    }
}
