use parser;

pub fn main() {
    let c = parser::Content::read_file(&"input.txt").expect("Input file not found!");
    let forest = day08::Forest::new(&c);
    println!("Part a: {}", forest.nbr_visible());
    println!("Part b: {}", forest.best_view_from_tree());
}

#[cfg(test)]
mod tests {
    use day08;
    use parser;


    #[test]
    fn t8_test() {
        let c = parser::Content::read(&"30373
25512
65332
33549
35390");
        let forest = day08::Forest::new(&c);
        assert_eq!(forest.nbr_visible(), 21)
    }

    #[test]
    fn t9_test() {
        let c = parser::Content::read(&"30373
25512
65332
33549
35390");
        let forest = day08::Forest::new(&c);
        assert_eq!(forest.nbr_visible(), 21)
    }

    #[test]
    fn t10_test() {
        let c = parser::Content::read(&"30373
25512
65332
33549
35390");
        let forest = day08::Forest::new(&c);
        assert_eq!(forest.visible_from_tree(1 as usize, 2 as usize), 4)
    }

    #[test]
    fn t11_test() {
        let c = parser::Content::read(&"30373
25512
65332
33549
35390");
        let forest = day08::Forest::new(&c);
        assert_eq!(forest.visible_from_tree(3 as usize, 2 as usize), 8)
    }

    #[test]
    fn t12_test() {
        let c = parser::Content::read(&"30373
25512
65332
33549
35390");
        let forest = day08::Forest::new(&c);
        assert_eq!(forest.visible_from_tree(1 as usize, 1 as usize), 1)
    }

    #[test]
    fn t13_test() {
        let c = parser::Content::read(&"30373
25512
65332
33549
35390");
        let forest = day08::Forest::new(&c);
        assert_eq!(forest.best_view_from_tree(), 8)
    }
}
