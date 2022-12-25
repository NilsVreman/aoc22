use parser;

const PATH: &str = "data";
const FILENAME: &str = "day22";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");

    let map  = day22::Map::new(&c);
    let list = day22::Instruction::new_list(&c);

    let res_a = day22::execute(&map, list.0, list.1);
    println!("Part A: {}", res_a);
}

#[cfg(test)]
mod tests {
    use day22;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5");
        let map  = day22::Map::new(&c);
        let list = day22::Instruction::new_list(&c);

        let res_a = day22::execute(&map, list.0, list.1);
        assert_eq!(res_a, 6032)

    }
}
