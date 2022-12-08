use parser;

pub fn main() {
    let c = parser::Content::read_file(&"input.txt").expect("No input file provided");

}

#[cfg(test)]
mod tests {
    use parser;
    use day09;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"");

    }
}
