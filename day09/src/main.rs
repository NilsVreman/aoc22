use parser;

pub fn main() {
    let c = parser::Content::read_file(&"input.txt").expect("No input file provided");
    let v = day09::Command::command_list(&c);
    println!("Part a: {}", day09::execute_command_list(&v).iter().count());

}

#[cfg(test)]
mod tests {
    use parser;
    use day09;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        let v = day09::Command::command_list(&c);
        assert_eq!(v.iter().filter(|&x| match x {
            &day09::Command::R => true,
            _ => false,
        }).count(), 10);
    }

    #[test]
    fn t2_test() {
        let c = parser::Content::read(&"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        let v = day09::Command::command_list(&c);
        assert_eq!(v.iter().filter(|&x| match x {
            &day09::Command::L => true,
            _ => false,
        }).count(), 8);
    }

    #[test]
    fn t3_test() {
        let c = parser::Content::read(&"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        let v = day09::Command::command_list(&c);
        assert_eq!(day09::execute_command_list(&v).iter().count(), 13);
    }
}
