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

#[derive(Clone, Copy)]
enum Facing {
    R,
    D,
    L,
    U,
}

impl Facing {
    fn direction(&self) -> (isize, isize) {
        match self {
            Facing::R => ( 0,  1),
            Facing::D => ( 1,  0),
            Facing::L => ( 0, -1),
            Facing::U => (-1,  0),
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

pub enum MapType {
    Linear,
    Cube,
}

pub struct Map {
    pub start: (isize, isize), // row, col
    map: Vec<Vec<u8>>, // row, col
    maptype: MapType,
}

impl Map {
    pub fn new(cont: &parser::Content, maptype: MapType) -> Self {
        let parts = cont.content.split("\n\n").collect::<Vec<_>>();

        let map = parts[0].lines()
            .map(|l| l.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let start: (isize, isize) = (0, parts[0].lines()
            .next()
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .unwrap() as isize);

        Self {
            map,
            start,
            maptype,
        }
    }

    fn next(&self, agent: &Agent) -> Option<((isize, isize), Facing)> {
        let dir         = agent.facing.direction();
        let mut new_pos = (agent.pos.0 + dir.0, agent.pos.1 + dir.1);

        match self.map.get(new_pos.0 as usize)
            .and_then(|row| row.get(new_pos.1 as usize))
            .unwrap_or(&b' ') {
                b'#' => Some((agent.pos, agent.facing)),
                b'.' => Some((new_pos, agent.facing)),
                b' ' => {
                    let (new_pos, new_dir) = match self.maptype {
                            MapType::Linear => self.wrap_linear(&agent),
                            MapType::Cube   => self.wrap_cube(&agent)
                    };
                    if self.map[new_pos.0 as usize][new_pos.1 as usize] == b'#' {
                        None
                    } else {
                        Some((new_pos, new_dir))
                    }
                },
                _    => unreachable!(),
        } 
    }

    // Find next pos for agent based on map layout
    fn wrap_linear(&self, agent: &Agent) -> ((isize, isize), Facing) {
        let dir         = agent.facing.direction();
        let mut new_pos = (agent.pos.0 + dir.0, agent.pos.1 + dir.1);

        while self.map.get((new_pos.0-dir.0) as usize)
            .and_then(|row| row.get((new_pos.1-dir.1) as usize))
            .unwrap_or(&b' ') != &b' ' {
                new_pos = (new_pos.0 - dir.0, new_pos.1 - dir.1);
        }

        (new_pos, agent.facing)
    }

    // Find next pos for agent based on map layout
    // NOTE: Hard coded based on my map!
    fn wrap_cube(&self, agent: &Agent) -> ((isize, isize), Facing) {
        let (npr, npc, new_facing) = match (agent.pos.0 / 50, agent.pos.1 / 50, &agent.facing) {
            (0, 1, &Facing::L) => (2, 0, Facing::R),
            (0, 1, &Facing::U) => (3, 0, Facing::R),
            (0, 2, &Facing::U) => (3, 0, Facing::U),
            (0, 2, &Facing::R) => (2, 1, Facing::L),
            (0, 2, &Facing::D) => (1, 1, Facing::L),
            (1, 1, &Facing::L) => (2, 0, Facing::D),
            (1, 1, &Facing::R) => (0, 2, Facing::U),
            (2, 0, &Facing::L) => (0, 1, Facing::R),
            (2, 0, &Facing::U) => (1, 1, Facing::R),
            (2, 1, &Facing::R) => (0, 2, Facing::L),
            (2, 1, &Facing::D) => (3, 0, Facing::L),
            (3, 0, &Facing::L) => (0, 1, Facing::D),
            (3, 0, &Facing::D) => (0, 2, Facing::D),
            (3, 0, &Facing::R) => (2, 1, Facing::U),
            _                 => unreachable!(),
        };
        let (dr, dc) = (agent.pos.0 % 50, agent.pos.1 % 50);
        let i = match agent.facing {
            Facing::U => dc,
            Facing::R => dr,
            Facing::D => 49-dc,
            Facing::L => 49-dr,
        };
        let (nr, nc) = match new_facing {
            Facing::U => (49, i),
            Facing::R => (i, 0),
            Facing::D => (0, 49-i),
            Facing::L => (49-i, 49),
        };
        ((npr*50 + nr, npc*50 + nc), new_facing)

        //match (&agent.facing, new_facing) {
        //    (&Facing::L, Facing::L) => (npr*50 + dr,     npc*50 + 49),
        //    (&Facing::U, Facing::L) => (npr*50 + 49-dc,  npc*50 + 49),
        //    (&Facing::R, Facing::L) => (npr*50 + 49-dr,  npc*50 + 49),
        //    (&Facing::D, Facing::L) => (npr*50 + dc,     npc*50 + 49),
        //    (&Facing::L, Facing::U) => (npr*50 + 49,     npc*50 + 49-dr),
        //    (&Facing::U, Facing::U) => (npr*50 + 49,     npc*50 + dc),
        //    (&Facing::R, Facing::U) => (npr*50 + 49,     npc*50 + dr),
        //    (&Facing::D, Facing::U) => (npr*50 + 49,     npc*50 + 49-dc),
        //    (&Facing::L, Facing::R) => (npr*50 + 49-dr,  npc*50 + 0),
        //    (&Facing::U, Facing::R) => (npr*50 + dc,     npc*50 + 0),
        //    (&Facing::R, Facing::R) => (npr*50 + dr,     npc*50 + 0),
        //    (&Facing::D, Facing::R) => (npr*50 + 49-dc,  npc*50 + 0),
        //    (&Facing::L, Facing::D) => (npr*50 + 0,      npc*50 + dr),
        //    (&Facing::U, Facing::D) => (npr*50 + 0,      npc*50 + 49-dc),
        //    (&Facing::R, Facing::D) => (npr*50 + 0,      npc*50 + 49-dr),
        //    (&Facing::D, Facing::D) => (npr*50 + 0,      npc*50 + dc),
        //}
    }

    // print function
    pub fn print(&self) {
        self.map.iter().for_each(|x| {
            x.iter().for_each(|y| {
                print!("{}", *y as char);
            });
            println!("");
        });
    }
}

pub struct Agent {
    pos: (isize, isize), // row, col
    facing: Facing,
}

impl Agent {
    pub fn new(pos: (isize, isize)) -> Self {
        Self { pos, facing: Facing::R }
    }

    pub fn walk(&mut self, map: &Map, n: isize) {
        for _ in 0..n {
            if let Some((new_pos, new_dir)) = map.next(&self) {
                self.pos    = new_pos;
                self.facing = new_dir;
            } else {
                break
            }
        }
    }

    pub fn rotate(&mut self, instr: &Instruction) {
        self.facing = self.facing.rotate(instr);
    }
}

pub fn execute(map: &Map, instr_list: &Vec<(isize, Instruction)>, last: &isize) -> isize {
    let mut agent = Agent::new(map.start);
    for (n, instr) in instr_list {
        agent.walk(&map, *n);
        agent.rotate(&instr);
    }
    agent.walk(&map, *last);
    4*(agent.pos.1+1) + 1000*(agent.pos.0+1) + agent.facing.value()
}
