use parser;

const PATH: &str = "data";
const FILENAME: &str = "day11";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");

}

#[cfg(test)]
mod tests {
    use day11;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"");

    }
}
