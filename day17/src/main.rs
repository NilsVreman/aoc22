use parser;

const PATH: &str = "../data";
const FILENAME: &str = "day17";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let jets = day17::Jet::new(&c);
    let res_a = day17::simulate(&jets, 2022);
    println!("Part A: {}", res_a);

    let res_b = day17::simulate(&jets, 1_000_000_000_000);
    println!("Part B: {}", res_b);
}

#[cfg(test)]
mod tests {
    use day17;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let jets = day17::Jet::new(&c);
        let height = day17::simulate(&jets, 2022);
        assert_eq!(height, 3068);
    }

    #[test]
    fn t2_test() {
        let c = parser::Content::read(&">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let jets = day17::Jet::new(&c);
        let height = day17::simulate(&jets, 1_000_000_000_000);
        assert_eq!(height, 1514285714288);
    }
}

