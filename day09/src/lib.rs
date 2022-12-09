use parser;

pub struct Head {
    x: i32,
    y: i32,
}

pub struct Tail {
    x: i32,
    y: i32,
}

#[derive(Clone)]
pub enum Command {
    L,
    R,
    U,
    D,
}

impl Command {
    pub fn command_list(c: &parser::Content) -> Vec<Command> {
        let ps = parser::Parser::build("{} {}", "{}");
        let mut output: Vec<Command> = Vec::new();
        c.content.lines()
            .map(|x| ps.parse(&x)
                 .unwrap_or_else(|err| panic!("Erroneous parsing {err}")))
            .for_each(|cmd| {
                output.append(&mut vec![match cmd[0] {
                    "L" => Command::L,
                    "R" => Command::R,
                    "U" => Command::U,
                    "D" => Command::D,
                    _   => panic!("Unknown character"),
                }; cmd[1].parse::<usize>().unwrap()])
            });
        output
    }
}

impl Head {
    pub fn new() -> Head {
        Head { x: 0, y: 0 }
    }

    pub fn execute(&mut self, cmd: &Command) {
        match cmd {
            Command::L => self.x -= 1,
            Command::R => self.x += 1,
            Command::D => self.y -= 1,
            Command::U => self.y += 1,
        };
    }
}

impl Tail {
    pub fn new() -> Tail {
        Tail { x: 0, y: 0 }
    }

    fn dist(&self, h: &Head) -> (i32, i32) {
        (h.x - self.x, h.y - self.y)
    }

    pub fn is_close(&self, h: &Head) -> bool {
        let d = self.dist(&h);
        d.0.abs() <= 1 && d.1.abs() <= 1
    }

    pub fn follow(&mut self, h: &Head) {
        if !self.is_close(h) {
            let d = self.dist(&h);
            if let 1 | 2 | -1 | -2 = d.0 {
                self.x += d.0.signum()*1;
            }
            if let 1 | 2 | -1 | -2 = d.1 {
                self.y += d.1.signum()*1;
            }
        }
    }
}

pub fn execute_command_list(list: &Vec<Command>) -> Vec<(i32, i32)> {
    let mut h = Head::new();
    let mut t = Tail::new();
    let mut res: Vec<(i32, i32)> = Vec::new();
    res.push((t.x, t.y));

    for cmd in list {
        h.execute(&cmd);
        t.follow(&h);
        res.push((t.x, t.y));
    }

    res.iter()
        .enumerate()
        .filter(|(i, x)| res.iter()
                .skip(i+1)
                .find(|y| x == y)
                .is_none())
        .map(|(_, &x)| x)
        .collect()
}
