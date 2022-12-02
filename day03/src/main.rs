use parser;

pub fn main() {
    let c = parser::Content::read_file(&"input.txt").expect("No such file found");

}

#[cfg(test)]
mod tests {
    use day03;

    #[test]
    fn t1_test() {
        
    }
}
