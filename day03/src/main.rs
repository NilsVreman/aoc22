use parser;

pub fn main() {
    let c = parser::Content::read_file(&"input.txt").expect("No such file found");
    let res = day03::rucksacks(&c)
        .iter()
        .map(|x| x.find_common())
        .sum::<u32>();
    println!("Priority: {}", res);
}

#[cfg(test)]
mod tests {
    use day03;

//    #[test]
//    fn t1_test() {
//        let input = parser::Content::read(&"vJrwpWtwJgWrhcsFMMfFFhFp
//jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//PmmdzqPrVvPwwTWBwg
//wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//ttgJtRGJQctTZtZT
//CrZsJsPPZsGzwwsLwLmpwMDw");
//        let res = day03::rucksacks(&input);
//        assert_eq!(res[0].c1, "vJrwpWtwJgWr")
//    }
//
//    #[test]
//    fn t2_test() {
//        let input = parser::Content::read(&"vJrwpWtwJgWrhcsFMMfFFhFp
//jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//PmmdzqPrVvPwwTWBwg
//wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//ttgJtRGJQctTZtZT
//CrZsJsPPZsGzwwsLwLmpwMDw");
//        let res = day03::rucksacks(&input);
//        assert_eq!(res[0].c2, "hcsFMMfFFhFp")
//    }

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
}
