use parser;

const PATH: &str = "../data";
const FILENAME: &str = "day17";

pub fn main() {
    //let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    //let jets = day17::Jet::new(&c);
    //let mut shaft = day17::Shaft::new();
    //let i = shaft.iterate(&jets);
    //for j in shaft.shaft.iter().take(shaft.top+6).rev() {
    //    println!("{}", j.iter().map(|&x| if x { '#' } else { '.' }).collect::<String>());
    //}
    //println!("Part A: {}", i);
}

#[cfg(test)]
mod tests {
    use day17;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let jets = day17::Jet::new(&c);
        let mut shaft = day17::Shaft::new();
        let i = shaft.iterate(&jets);
        for j in shaft.shaft.iter().take(shaft.top+6).rev() {
            println!("{}", j.iter().map(|&x| if x { '#' } else { '.' }).collect::<String>());
        }
        assert_eq!(i,  3068);
    }
}
