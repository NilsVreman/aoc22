use parser;

const PATH: &str = "data";
const FILENAME: &str = "day02";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let ps = parser::Parser::build("{} {}", "{}");
    let res_a: u32 = c.content
        .lines()
        .map(|x| day02::game(&ps.parse(x).expect("Invalid parsing")))
        .sum();
    //println!("Part A: {}", res_a);

    let res_b: u32 = c.content
        .lines()
        .map(|x| day02::game_from_answer(&ps.parse(x).expect("Invalid parsing")))
        .sum();
    //println!("Part B: {}", res_b);
}

#[cfg(test)]
mod tests {
    use parser;
    use day02;

    #[test]
    fn t1_test() {
        let input = "A Y
B X
C Z";
        let ps = parser::Parser::build("{} {}", "{}");
        let res: Vec<u32> = input.lines().map(|x| day02::game(&ps.parse(x).expect("Invalid parsing"))).collect();
        let cmp: Vec<u32> = vec![8, 1, 6];

        assert_eq!(res, cmp)
    }

    #[test]
    fn t2_test() {
        let input = "A Y
B X
C Z";
        let ps = parser::Parser::build("{} {}", "{}");
        let res: Vec<u32> = input.lines().map(|x| day02::game_from_answer(&ps.parse(x).expect("Invalid parsing"))).collect();
        let cmp: Vec<u32> = vec![4, 1, 7];

        assert_eq!(res, cmp)
    }

}
