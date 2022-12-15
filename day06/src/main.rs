use parser;

const PATH: &str = "data";
const FILENAME: &str = "day06";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    //println!("Part A: {}", day06::find_marker(&c));
    //println!("Part B: {}", day06::find_msg_marker(&c));
}

#[cfg(test)]
mod tests {
    use day06;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(day06::find_marker(&c), 7);
    }

    #[test]
    fn t2_test() {
        let c = parser::Content::read(&"bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(day06::find_marker(&c), 5);
    }

    #[test]
    fn t3_test() {
        let c = parser::Content::read(&"nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(day06::find_marker(&c), 6);
    }

    #[test]
    fn t4_test() {
        let c = parser::Content::read(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(day06::find_marker(&c), 10);
    }

    #[test]
    fn t5_test() {
        let c = parser::Content::read(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(day06::find_marker(&c), 11);
    }

    #[test]
    fn t6_test() {
        let c = parser::Content::read(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(day06::find_msg_marker(&c), 19);
    }

    #[test]
    fn t7_test() {
        let c = parser::Content::read(&"bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(day06::find_msg_marker(&c), 23);
    }

    #[test]
    fn t8_test() {
        let c = parser::Content::read(&"nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(day06::find_msg_marker(&c), 23);
    }

    #[test]
    fn t9_test() {
        let c = parser::Content::read(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(day06::find_msg_marker(&c), 29);
    }

    #[test]
    fn t10_test() {
        let c = parser::Content::read(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(day06::find_msg_marker(&c), 26);
    }
}

