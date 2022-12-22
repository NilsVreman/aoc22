use std::collections::HashSet;
use parser;

#[derive(Debug)]
pub enum Instruction {
    L,
    R,
}

impl Instruction {
    pub fn new(s: &str) -> Self {
        match s {
            "L" => Instruction::L,
            "R" => Instruction::R,
            _   => panic!("Unknown char in input"),
        }
    }

    pub fn new_list(cont: &parser::Content) -> (Vec<(isize, Instruction)>, isize) {
        let parts   = cont.content.split("\n\n").collect::<Vec<_>>();
        let nbrs    = parts[1].trim().split(char::is_alphabetic).map(|x| x.parse::<isize>().unwrap());
        let instrs  = parts[1].trim().split(char::is_numeric).filter(|x| !x.is_empty()).map(|x| Instruction::new(x));
        let v       = nbrs.zip(instrs).collect::<Vec<_>>();
        let last    = parts[1].trim().split(char::is_alphabetic).last().unwrap().parse::<isize>().unwrap();
        (v, last)
    }
}

enum Facing {
    R,
    D,
    L,
    U,
}

impl Facing {
    fn direction(&self) -> (isize, isize) {
        match self {
            Facing::R => ( 1,  0),
            Facing::D => ( 0,  1),
            Facing::L => (-1,  0),
            Facing::U => ( 0, -1),
        }
    }

    fn rotate(&self, instr: &Instruction) -> Facing {
        match (self, instr) {
            (Facing::R, Instruction::L) => Facing::U,
            (Facing::R, Instruction::R) => Facing::D,
            (Facing::D, Instruction::L) => Facing::R,
            (Facing::D, Instruction::R) => Facing::L,
            (Facing::L, Instruction::L) => Facing::D,
            (Facing::L, Instruction::R) => Facing::U,
            (Facing::U, Instruction::L) => Facing::L,
            (Facing::U, Instruction::R) => Facing::R,
        }
    }

    fn value(&self) -> isize {
        match self {
            Facing::R => 0,
            Facing::D => 1,
            Facing::L => 2,
            Facing::U => 3,
        }
    }
}

pub struct Map {
    pub start: (isize, isize), // col, row
    fields: HashSet<(isize, isize)>, // col, row
    edges: HashSet<(isize, isize)>, // col, row
    max_col: isize,
    max_row: isize,
}

impl Map {
    pub fn new(cont: &parser::Content) -> Self {
        let mut fields: HashSet<(isize, isize)> = HashSet::new();
        let mut edges:   HashSet<(isize, isize)> = HashSet::new();

        let parts = cont.content.split("\n\n").collect::<Vec<_>>();
        let mut max_col: isize = 0;
        parts[0].lines().for_each(|l| {
            if l.len() as isize > max_col { max_col = l.len() as isize; }
        });

        let mut row: isize = 0;
        let mut col: isize = 0;
        let mut start: (isize, isize) = (0, 0);

        parts[0].lines()
            .for_each(|l| {
                row += 1;
                col = 1;
                l.chars().for_each(|c| {
                    match c {
                        '#' => { fields.insert((col, row)); },  // If we found a force field
                        ' ' => { edges.insert((col, row)); },    // if we found a space outside the map
                        '.' => { if row == 1 && start == (0, 0) {
                            start = (col, row);
                        }},
                        _   => panic!("What is this strange character: {c}")
                    };
                    col += 1;
                });
                for c in col..=max_col {
                    edges.insert((c, row));
                }
            });

        for c in 0..=max_col {
            edges.insert((c, 0));
            edges.insert((c, row+1));
        }
        for r in 0..=row+1 {
            edges.insert((0, r));
            edges.insert((max_col+1, r));
        }

        Self {
            fields: fields,
            edges: edges,
            start: start,
            max_col: max_col,
            max_row: row+1
        }
    }

    pub fn feasible(&self, pos: (isize, isize)) -> bool {
        !self.fields.contains(&pos)
    }

    pub fn outside(&self, pos: (isize, isize)) -> bool {
        if pos.0 < 0 || pos.0 > self.max_col+1 || pos.1 < 0 || pos.1 > self.max_row+1 {
            return true
        }
        self.edges.contains(&pos)
    }

    // private function to find next pos for agent in map
    fn iter_horizontal<T>(&self, agent: &Agent, iter: T) -> (isize, isize) 
        where T: Iterator<Item=isize>, // the iterator of col values
    {
        for c in iter {
            if !self.outside((c, agent.pos.1)) {
                if self.feasible((c, agent.pos.1)) {
                    return (c, agent.pos.1)
                } else {
                    return agent.pos
                }
            }
        }
        panic!("How is this possible?")
    }

    // private function to find next pos for agent in map
    fn iter_vertical<T>(&self, agent: &Agent, iter: T) -> (isize, isize) 
        where T: Iterator<Item=isize>, // the iterator of row values
    {
        for r in iter {
            if !self.outside((agent.pos.0, r)) {
                if self.feasible((agent.pos.0, r)) {
                    return (agent.pos.0, r)
                } else {
                    return agent.pos
                }
            }
        }
        panic!("How is this possible?")
    }

    // Find next pos for agent based on map layout
    pub fn next(&self, agent: &Agent) -> (isize, isize) {
        let dir = agent.facing.direction();
        let mut new_pos = (agent.pos.0 + dir.0, agent.pos.1 + dir.1);

        match (!self.outside(new_pos), self.feasible(new_pos)) {
            (true, true)    => new_pos,
            (true, false)   => agent.pos,
            (false, _)      => {
                match agent.facing {
                    Facing::R => { // iterate from left to right 
                        self.iter_horizontal(agent, 1..=agent.pos.0)
                    },
                    Facing::D => { // iterate from up to down 
                        self.iter_vertical(agent, 1..=agent.pos.1)
                    },
                    Facing::L => { // iterate from right to left 
                        self.iter_horizontal(agent, (agent.pos.0..=self.max_col).rev())
                    },
                    Facing::U => { // iterate from down to up 
                        self.iter_vertical(agent, (agent.pos.1..=self.max_row).rev())
                    },
                }
            }
        }
    }

    // print function
    pub fn print(&self) {
        for r in 0..=self.max_row {
            for c in 0..=self.max_col+1 {
                if self.outside((c, r)) {
                    print!("x");
                } else if !self.feasible((c, r)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

pub struct Agent {
    pub pos: (isize, isize),
    facing: Facing,
}

impl Agent {
    pub fn new(pos: (isize, isize)) -> Self {
        Self { pos: pos, facing: Facing::R }
    }

    pub fn walk(&mut self, map: &Map, n: isize) {
        for _ in 0..n {
            let new_pos = map.next(&self);
            if new_pos == self.pos {
                break
            }
            self.pos = new_pos;
        }
    }

    pub fn rotate(&mut self, instr: &Instruction) {
        self.facing = self.facing.rotate(instr);
    }
}

pub fn execute(map: &Map, instr_list: Vec<(isize, Instruction)>, last: isize) -> isize {
    let mut agent = Agent::new(map.start);
    for (n, instr) in instr_list {
        agent.walk(&map, n);
        agent.rotate(&instr);
    }
    agent.walk(&map, last);
    4*agent.pos.0 + 1000*agent.pos.1 + agent.facing.value()
}
