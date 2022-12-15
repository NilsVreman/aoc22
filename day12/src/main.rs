use parser;

const PATH: &str = "data";
const FILENAME: &str = "day12";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let g = day12::Grid::new(&c);
    //println!("Part A: {}", g.shortest_path()[g.start.0][g.start.1]);
    //println!("Part B: {}", g.shortest_scenic_path());
}

#[cfg(test)]
mod tests {
    use day12;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi");
        let g = day12::Grid::new(&c);
        assert_eq!(g.shortest_path()[g.start.0][g.start.1], 31)
    }
}
