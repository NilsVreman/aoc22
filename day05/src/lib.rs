use parser;

pub struct CargoMove {
    from: usize,
    to: usize,
    n: usize,
}

impl CargoMove {
    pub fn build_move_stack(commands: &str) -> Vec<CargoMove> {
        let ps = parser::Parser::build(&"move {} from {} to {}", "{}");
        commands.lines().map(|x| match ps.parse(&x) {
                Ok(s) => CargoMove { 
                    from: s[1].parse::<usize>().unwrap()-1,
                    to: s[2].parse::<usize>().unwrap()-1,
                    n: s[0].parse::<usize>().unwrap(),
                },
                _ => panic!("Couldn't parse command input")
            })
            .collect()
    }
}

pub struct Stacks<'a> {
    stacks: Vec<Vec<&'a str>>,
}

impl<'a> Stacks<'a> {
    pub fn build(config: &'a str) -> Stacks<'a> {
        let n = config.lines().next().unwrap().len()/4 + 1;
        let mut stacks: Vec<Vec<&str>> = (0..n).map(|_x| Vec::new()).collect();
        let mut j = 0;

        for line in config.lines().rev().skip(1) {
            for i in 0..line.len() {
                if i % 4 == 0 {
                    stacks[j].push(&line[i..i+3].trim());
                    j += 1;
                }
            }
            j = 0;
        }

        for i in 0..stacks.len() {
            stacks[i] = stacks[i].iter()
                .filter(|x| !x.is_empty())
                .map(|x| &x[1..2] )
                .collect();
        }

        Stacks { stacks }
    }

    pub fn execute_9000(&mut self, cmd: &Vec<CargoMove>) {
        cmd.iter().for_each(|x| {
            for _ in 0..x.n {
                let v = self.stacks[x.from].pop().expect("Can't pop a value when there is none");
                self.stacks[x.to].push(v)
            }
        })
    }

    pub fn execute_9001(&mut self, cmd: &Vec<CargoMove>) {
        cmd.iter().for_each(|x| {
            let mut tmp = Vec::new();
            for _ in 0..x.n {
                let v = self.stacks[x.from].pop().expect("Can't pop a value when there is none");
                tmp.push(v);
            }
            tmp.reverse();
            self.stacks[x.to].append(&mut tmp);
        })
    }

    pub fn top_layer(&self) -> String {
        self.stacks.iter().fold("".to_string(), |acc, x| acc + x.last().unwrap())
    }
}
