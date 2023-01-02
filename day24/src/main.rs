use parser;

const PATH: &str = "data";
const FILENAME: &str = "day24";

pub fn main() {
    let cont = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let mut grid = day24::Grid::new(&cont);
    let res_a = grid.travel();
    //println!("Part A: {}", res_a);

    let mut grid = day24::Grid::new(&cont);
    let res_b = grid.travel_twice();
    //println!("Part B: {}", res_b);
}

#[cfg(test)]
mod tests {
    use day24;
    use parser;

    #[test]
    fn t1_test() {
        let cont = parser::Content::read("#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#");
        let mut grid = day24::Grid::new(&cont);
        assert_eq!(grid.travel(), 18)
    }

    #[test]
    fn t2_test() {
        let cont = parser::Content::read("#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#");
        let mut grid = day24::Grid::new(&cont);
        assert_eq!(grid.travel_twice(), 54)
    }
}
