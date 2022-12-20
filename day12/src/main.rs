use parser;

const PATH: &str = "data";
const FILENAME: &str = "day12";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let g = day12::Grid::new(&c);
    let res_a = g.shortest_path()[g.start.0][g.start.1];
    let res_b = g.shortest_scenic_path();
    //println!("Part A: {}", res_a);
    //println!("Part B: {}", res_b);
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
