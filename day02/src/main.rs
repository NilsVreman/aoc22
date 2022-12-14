use parser;

const PATH: &str = "data";
const FILENAME: &str = "day02";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let ps = parser::Parser::build("{} {}", "{}");
    let res: u32 = c.content
        .lines()
        .map(|x| day02::game(&ps.parse(x).expect("Invalid parsing")))
        .sum();
    println!("Res exc 1: {}", res);

    let res: u32 = c.content
        .lines()
        .map(|x| day02::game_from_answer(&ps.parse(x).expect("Invalid parsing")))
        .sum();
    println!("Res exc 2: {}", res);
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
