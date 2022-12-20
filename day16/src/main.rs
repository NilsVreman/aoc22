use parser;

const PATH: &str = "data";
const FILENAME: &str = "day16";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let g = day16::Grid::new(&c);
    let state = day16::State::new(0, 30);
    let res_a = g.max_pressure_release(&state);
    //println!("Part A: {}", res_a);

    let state = day16::ElephantState::new(0, 26);
    let res_b = g.max_pressure_release_w_elephant(&state);
    //println!("Part B: {}", res_b);
}

#[cfg(test)]
mod tests {
    use day16;
    use parser;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II");
        let g = day16::Grid::new(&c);
        let state = day16::State::new(0, 30);
        assert_eq!(g.max_pressure_release(&state), 1651)
    }

    #[test]
    fn t2_test() {
        let c = parser::Content::read(&"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II");
        let g = day16::Grid::new(&c);
        let state = day16::ElephantState::new(0, 26);
        assert_eq!(g.max_pressure_release_w_elephant(&state), 1707)
    }
}
