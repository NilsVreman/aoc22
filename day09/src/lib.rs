use parser;
use std::collections::HashMap;

#[derive(Copy)]
#[derive(Clone)]
pub struct Node {
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

impl Node {
    pub fn new() -> Node {
        Node { x: 0, y: 0 }
    }

    pub fn execute(&mut self, cmd: &Command) {
        match cmd {
            Command::L => self.x -= 1,
            Command::R => self.x += 1,
            Command::D => self.y -= 1,
            Command::U => self.y += 1,
        };
    }

    fn dist(&self, h: &Node) -> (i32, i32) {
        (h.x - self.x, h.y - self.y)
    }

    pub fn is_close(&self, h: &Node) -> bool {
        let d = self.dist(&h);
        d.0.abs() <= 1 && d.1.abs() <= 1
    }

    pub fn follow(&mut self, h: &Node) {
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

pub fn execute_command_list<const TAIL_LEN: usize>(list: &Vec<Command>) -> usize {
    let mut head = Node::new();
    let mut tail = [Node::new(); TAIL_LEN];
    let mut res: HashMap<(i32, i32), bool> = HashMap::new();
    res.insert((tail[TAIL_LEN-1].x, tail[TAIL_LEN-1].y), true);

    for cmd in list {
        head.execute(&cmd);
        tail[0].follow(&head);
        for i in 1..TAIL_LEN {
            tail[i].follow(&tail[i-1].clone());
        }
        res.insert((tail[TAIL_LEN-1].x, tail[TAIL_LEN-1].y), true);
    }

    res.into_keys().count()
}
