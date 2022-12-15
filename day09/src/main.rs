use parser;

const PATH: &str = "data";
const FILENAME: &str = "day09";
const TAIL_LEN_A: usize = 1;
const TAIL_LEN_B: usize = 9;

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let v = day09::Command::command_list(&c);

    //println!("Part A: {}", day09::execute_command_list::<TAIL_LEN_A>(&v));
    //println!("Part B: {}", day09::execute_command_list::<TAIL_LEN_B>(&v));
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
        const TAIL_LEN: usize = 1;
        let c = parser::Content::read(&"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        let v = day09::Command::command_list(&c);
        assert_eq!(day09::execute_command_list::<TAIL_LEN>(&v), 13);
    }

    #[test]
    fn t4_test() {
        const TAIL_LEN: usize = 9;
        let c = parser::Content::read(&"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        let v = day09::Command::command_list(&c);
        assert_eq!(day09::execute_command_list::<TAIL_LEN>(&v), 1);
    }

    #[test]
    fn t5_test() {
        const TAIL_LEN: usize = 9;
        let c = parser::Content::read(&"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
        let v = day09::Command::command_list(&c);
        assert_eq!(day09::execute_command_list::<TAIL_LEN>(&v), 36);
    }
}
