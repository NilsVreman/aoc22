use parser;
use day13::{Packet};

const PATH: &str = "data";
const FILENAME: &str = "day13";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");

    let res = c.content.split("\n\n")
             .map(|pair| {
                 let mut itr = pair.lines();
                 let a = Packet::build(itr.next().unwrap());
                 let b = Packet::build(itr.next().unwrap());
                (a, b)
            }).enumerate()
            .filter(|(_, (a, b))| a < b)
            .map(|(i, _)| i + 1)
            .sum::<usize>();
    //println!("Part A: {}", res);
    
    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    let packets: Vec<Packet> = c.content.lines()
        .filter(|line| !line.is_empty())
        .map(Packet::build)
        .collect();

    // Count the number of packets that are correctly ordered w.r.t divider1 & 2.
    let idx1 = packets.iter().filter(|&packet| packet < &divider1).count();
    let idx2 = packets.iter().filter(|&packet| packet < &divider2).count();

    //println!("Part B: {}", (idx1+1)*(idx2+2));
}
