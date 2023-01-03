use parser;

const PATH: &str = "data";
const FILENAME: &str = "day22";

pub fn main() {
    let cont = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let list = day22::Instruction::new_list(&cont);

    let map  = day22::Map::new(&cont, day22::MapType::Linear);
    let res_a = day22::execute(&map, &list.0, &list.1);
    //println!("Part A: {}", res_a);

    let map  = day22::Map::new(&cont, day22::MapType::Cube);
    let res_b = day22::execute(&map, &list.0, &list.1);
    //println!("Part B: {}", res_b);
}

#[cfg(test)]
mod tests {
    use day22;
    use parser;

    #[test]
    fn t1_test() {
        let cont = parser::Content::read(&"        ...#
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
        let map  = day22::Map::new(&cont, day22::MapType::Linear);
        let list = day22::Instruction::new_list(&cont);

        let res_a = day22::execute(&map, list.0, list.1);
        assert_eq!(res_a, 6032)
    }
}
