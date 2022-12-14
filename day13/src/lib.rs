use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
pub enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l.cmp(r), // Vec implements Ord.
            (Packet::Int(l), r) => Packet::List(vec![Packet::Int(*l)]).cmp(r),
            (l, Packet::Int(r)) => l.cmp(&Packet::List(vec![Packet::Int(*r)])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    pub fn build(input: &str) -> Packet {
        let chars: Vec<char> = input.chars().collect();
        Packet::parse_sub(&chars[..], &mut 0)
    }

    fn parse_sub(part: &[char], idx: &mut usize) -> Packet {
        let mut contents = Vec::new();
        let mut acc: Option<u32> = None;

        loop {
            *idx += 1;
            match part[*idx] {
                '0'..='9' => acc = Some(
                    match acc {
                        Some(x) => 10 * x,
                        None => 0,
                    } + part[*idx].to_digit(10).unwrap()
                ),
                '[' => contents.push(Packet::parse_sub(part, idx)),
                ']' => {
                    if let Some(x) = acc {
                        contents.push(Packet::Int(x));
                    }
                    return Packet::List(contents)
                },
                ',' => if let Some(x) = acc {
                    contents.push(Packet::Int(x));
                    acc = None;
                },
                _ => panic!("Invalid character encountered.")
            }
        }
    }
}
