use parser::Content;

pub enum Instr {
    Add(i32),
    Noop,
}

impl Instr {
    pub fn instr_list(c: &Content) -> Vec<Instr> {
        c.content.lines().map(|x| {
            if x.contains("noop") {
                Instr::Noop
            } else {
                Instr::Add(x.split(' ').skip(1).next().unwrap().parse::<i32>().unwrap()) // get 2nd element
            }
        }).collect()
    }

    pub fn cycles(&self) -> usize {
        match self {
            Instr::Add(_) => 2,
            Instr::Noop => 1,
        }
    }

    pub fn update_registry(&self, x: &mut i32) {
        *x += match self {
            Instr::Add(v) => v,
            Instr::Noop => &0,
        };
    }
}

pub fn register_list(list: &Vec<Instr>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut x = 1;
    let mut cycle = 0;
    for instr in list {
        for _ in 0..instr.cycles() {
            cycle +=1;
            res.push(x);
        }
        instr.update_registry(&mut x);
    }
    res
}

pub fn signal_str_at(registers: &Vec<i32>, during_list: &Vec<usize>) -> Vec<i32> {
    during_list.iter().map(|&x| registers[x-1]*(x as i32)).collect()
}

pub fn pixels(registers: &Vec<i32>, size_h: usize, size_v: usize) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![vec!['.'; size_h]; size_v];
    let size = size_h*size_v;
    for i in 0..size {
        let r = i / 40;
        let c = i % 40;
        res[r][c] = match registers[i] {
            x if x >= c as i32 - 1 && x <= c as i32 + 1 => '#',
            _ => '.',
        };
    }
    res
}
