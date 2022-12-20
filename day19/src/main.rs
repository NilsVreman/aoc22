use parser;
use std::collections::HashMap;

const PATH: &str = "data";
const FILENAME: &str = "day19";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");

    let mut map: HashMap<day19::State, u16> = HashMap::new();
    let s       = day19::State::new(24);
    let mut sum = 0;

    for l in c.content.lines() {
        map.clear();
        let bp = day19::Blueprint::new(l);
        let g = day19::execute_bp(&bp, &s, &mut map);
        sum += g*bp.id;
    }
    //println!("Part A: {}", sum);

    let s   = day19::State::new(32);
    sum     = 1;
    for l in c.content.lines().take(3) {
        map.clear();
        let bp = day19::Blueprint::new(l);
        let g = day19::execute_bp(&bp, &s, &mut map);
        sum *= g;
    }
    //println!("Part B: {}", sum);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use parser;
    use day19;

    #[test]
    fn t1_test() {
        println!("What");
        let c = parser::Content::read(&"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
        let mut map: HashMap<day19::State, u16> = HashMap::new();
        let s = day19::State::new(24);
        let bp = day19::Blueprint::new(c.content.lines().next().unwrap());
        println!("{:?}", bp);
        assert_eq!(day19::execute_bp(&bp, &s, &mut map), 9)
    }

    #[test]
    fn t2_test() {
        println!("What");
        let c = parser::Content::read(&"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
        let mut map: HashMap<day19::State, u16> = HashMap::new();
        let s = day19::State::new(32);
        let bp = day19::Blueprint::new(c.content.lines().skip(1).next().unwrap());
        println!("{:?}", bp);
        assert_eq!(day19::execute_bp(&bp, &s, &mut map), 62)
    }
}
