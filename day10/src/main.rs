use parser;

pub fn main() {
    let c = parser::Content::read_file(&"input.txt").expect("No input file found!");

}

#[cfg(test)]
mod tests {
    use day10;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"");

    }
}
