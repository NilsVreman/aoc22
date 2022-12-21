use parser;

const PATH: &str = "data";
const FILENAME: &str = "day21";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let jobs = day21::get_jobs(&c);
    let res_a = day21::execute(&jobs);
    //println!("Part A: {}", res_a);

    let res_b = day21::execute_with_mistakes(&jobs);
    //println!("Part B: {}", res_b);
}

#[cfg(test)]
mod tests {
    use day21;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32");
        let jobs = day21::get_jobs(&c);
        assert_eq!(day21::execute(&jobs), 152)
    }

    #[test]
    fn t2_test() {
        let c = parser::Content::read(&"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32");
        let jobs = day21::get_jobs(&c);
        assert_eq!(day21::execute_with_mistakes(&jobs), 301)
    }
}
