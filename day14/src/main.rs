use parser;

const PATH: &str = "data";
const FILENAME: &str = "day14";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let mut w = day14::Wall::new(&c);
    let lowest = w.lowest;
    w.flow_sand();
    println!("Part A: {}", w.paths.values().filter(|&x| *x == day14::Tile::Sand).count());

    let mut w = day14::Wall::new(&c);
    w.flow_sand_with_floor();
    println!("Part B: {}", w.paths.values().filter(|&x| *x == day14::Tile::Sand).count());
}

#[cfg(test)]
mod tests {
    use day14;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9");
        let mut w = day14::Wall::new(&c);
        while w.flow_sand() {}
        assert_eq!(w.paths.values().filter(|&x| *x == day14::Tile::Sand).count(), 24);
    }

    #[test]
    fn t2_test() {
        let c = parser::Content::read(&"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9");
        let mut w = day14::Wall::new(&c);
        while w.flow_sand_with_floor() {}
        assert_eq!(w.paths.values().filter(|&x| *x == day14::Tile::Sand).count(), 93);
    }
}
