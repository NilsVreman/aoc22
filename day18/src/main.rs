use parser;

const PATH: &str = "data";
const FILENAME: &str = "day18";

pub fn main() {
    let cont = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let mut grid = day18::Grid::new(&cont);
    let res_a = grid.total_area();
    println!("Part A: {}", res_a);

    let res_b = grid.visible_area(Some(res_a));
    println!("Part B: {}", res_b);
}

#[cfg(test)]
mod tests {
    use day18;
    use parser;

    #[test]
    fn t1_test() {
        let cont = parser::Content::read(&"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5");
        let grid = day18::Grid::new(&cont);
        assert_eq!(grid.total_area(), 64)
    }

    #[test]
    fn t2_test() {
        let cont = parser::Content::read(&"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5");
        let mut grid = day18::Grid::new(&cont);
        assert_eq!(grid.visible_area(None), 58)
    }
}
