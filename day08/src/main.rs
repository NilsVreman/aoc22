use parser;

const PATH: &str = "data";
const FILENAME: &str = "day08";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let forest = day08::Forest::new(&c);
    //println!("Part A: {}", forest.nbr_visible());
    //println!("Part B: {}", forest.best_view_from_tree());
}

#[cfg(test)]
mod tests {
    use day08;
    use parser;


    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"30373
25512
65332
33549
35390");
        let forest = day08::Forest::new(&c);
        assert_eq!(forest.nbr_visible(), 21)
    }

    #[test]
    fn t2_test() {
        let c = parser::Content::read(&"30373
25512
65332
33549
35390");
        let forest = day08::Forest::new(&c);
        assert_eq!(forest.best_view(), 8)
    }
}
