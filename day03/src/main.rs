use parser;

const PATH: &str = "data";
const FILENAME: &str = "day03";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let r = day03::rucksacks(&c);
    let res = r.iter()
        .map(|x| x.find_common())
        .sum::<u32>();
    //println!("Part A: {}", res);

    let mut res2: u32 = 0;
    for i in (0..r.len()-2).step_by(3) {
        res2 += day03::find_common_group(&r[i..i+3]);
    }
    //println!("Part B: {}", res2);
}

#[cfg(test)]
mod tests {
    use day03;


    #[test]
    fn t3_test() {
        let input = parser::Content::read(&"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
        let res = day03::rucksacks(&input);
        assert_eq!(res[0].find_common(), 16 as u32);
    }

    #[test]
    fn t4_test() {
        let input = parser::Content::read(&"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
        let res = day03::rucksacks(&input);
        assert_eq!(res[1].find_common(), 38 as u32);
    }

    #[test]
    fn t5_test() {
        let input = parser::Content::read(&"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
        let res = day03::rucksacks(&input);
        assert_eq!(res[2].find_common(), 42 as u32);
    }

    #[test]
    fn t6_test() {
        let input = parser::Content::read(&"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
        let res = day03::rucksacks(&input);
        assert_eq!(res[3].find_common(), 22 as u32);
    }

    #[test]
    fn t7_test() {
        let input = parser::Content::read(&"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
        let res = day03::rucksacks(&input);
        assert_eq!(res[4].find_common(), 20 as u32);
    }

    #[test]
    fn t8_test() {
        let input = parser::Content::read(&"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
        let res = day03::rucksacks(&input);
        assert_eq!(res[5].find_common(), 19 as u32);
    }

    #[test]
    fn t9_test() {
        let input = parser::Content::read(&"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
        let res = day03::rucksacks(&input);
        assert_eq!(res.iter().map(|x| x.find_common()).sum::<u32>(), 157);
    }

    #[test]
    fn t10_test() {
        let input = parser::Content::read(&"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
        let res = day03::rucksacks(&input);
        let mut res2: u32 = 0;
        for i in (0..res.len()-2).step_by(3) {
            res2 += day03::find_common_group(&res[i..i+3]);
            println!("Priority: {}", res2);
        }
        assert_eq!(res.iter().map(|x| x.find_common()).sum::<u32>(), 157);
    }
}
