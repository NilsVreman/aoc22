use parser;

const PATH: &str = "data";
const FILENAME: &str = "day23";

pub fn main() {
    let cont = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let mut grid = day23::Grid::new(&cont);
    let mut dirs: Vec<day23::Dir> = vec![day23::Dir::N, day23::Dir::S, day23::Dir::W, day23::Dir::E];
    let (area, n) = grid.stationary(&mut dirs, 10);
    println!("Part A: {}", area);

    let mut grid = day23::Grid::new(&cont);
    let mut dirs: Vec<day23::Dir> = vec![day23::Dir::N, day23::Dir::S, day23::Dir::W, day23::Dir::E];
    let (area, n) = grid.stationary(&mut dirs, 10_000);
    println!("Part B: {}", n);
}

#[cfg(test)]
mod tests {
    use day23;
    use parser;

    #[test]
    fn t0_test() {
    let cont = parser::Content::read(&".....
..##.
..#..
.....
..##.
.....");
        let mut grid = day23::Grid::new(&cont);
        let mut dirs: Vec<day23::Dir> = vec![day23::Dir::N, day23::Dir::S, day23::Dir::W, day23::Dir::E];
        let area = grid.stationary(&mut dirs, 10);
        assert_eq!(area, 25)
    }

    #[test]
    fn t1_test() {
        let cont = parser::Content::read(&"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............");
        let mut grid = day23::Grid::new(&cont);
        let mut dirs: Vec<day23::Dir> = vec![day23::Dir::N, day23::Dir::S, day23::Dir::W, day23::Dir::E];
        let area = grid.stationary(&mut dirs, 10);
        assert_eq!(area, 110)
    }
}
