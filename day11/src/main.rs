use num::integer;
use parser;

const PATH: &str = "data";
const FILENAME: &str = "day11";

pub fn main() {
    let cont = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let mut monkeys = day11::Monkey::monkey_list(&cont);
    let res_a = day11::rounds(monkeys, 20, |x| x / 3);
    //println!("Part A: {}", res_a);

    let mut monkeys = day11::Monkey::monkey_list(&cont);
    let lcm = monkeys.iter().fold(1, |acc, x| integer::lcm(acc, x.divisor));
    let res_b = day11::rounds(monkeys, 10_000, |x| x % lcm);
    //println!("Part B: {}", res_b);
}

#[cfg(test)]
mod tests {
    use day11;
    use parser;

    #[test]
    fn t1_test() {
        let cont = parser::Content::read(&"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1");
        let mut monkeys = day11::Monkey::monkey_list(&cont);
        let res = day11::rounds(monkeys, 20, |x| x / 3);
        assert_eq!(res, 10605)
    }

    #[test]
    fn t2_test() {
        let cont = parser::Content::read(&"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1");
        let mut monkeys = day11::Monkey::monkey_list(&cont);
        let res = day11::rounds_worry(monkeys, 10_000);
        assert_eq!(res, 2713310158)
    }
}
