use parser;

const PATH: &str = "data";
const FILENAME: &str = "day10";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let d: Vec<usize> = vec![20, 60, 100, 140, 180, 220];

    let l = day10::Instr::instr_list(&c);
    let v = day10::register_list(&l);
    let s = day10::signal_str_at(&v, &d);
    //println!("Part A: {}", s.iter().sum::<i32>());

    let p = day10::pixels(&v, 40, 6);
    //println!("Part B:");
    //p.iter().for_each(|x| println!("{}", String::from_iter(x)));
}

#[cfg(test)]
mod tests {
    use day10;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop");
        let l = day10::Instr::instr_list(&c);
        let v = day10::signal_str(&l);
        let d: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
        let s = day10::signal_str_at(&v, &d);
        assert_eq!(s, vec![420, 1140, 1800, 2940, 2880, 3960])
    }
}
