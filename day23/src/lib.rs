use std::collections::{HashSet, HashMap};
use parser;

pub enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    pub fn get_positions(&self, pos: &(isize, isize)) -> [(isize, isize); 3] {
        match self {
            Dir::N  => [(pos.0-1, pos.1-1), (pos.0, pos.1-1), (pos.0+1, pos.1-1)],
            Dir::S  => [(pos.0-1, pos.1+1), (pos.0, pos.1+1), (pos.0+1, pos.1+1)],
            Dir::W  => [(pos.0-1, pos.1-1), (pos.0-1, pos.1), (pos.0-1, pos.1+1)],
            Dir::E  => [(pos.0+1, pos.1-1), (pos.0+1, pos.1), (pos.0+1, pos.1+1)],
        }
    }

    pub fn go(&self, pos: &(isize, isize)) -> (isize, isize) {
        match self {
            Dir::N  => (pos.0, pos.1-1),
            Dir::S  => (pos.0, pos.1+1),
            Dir::W  => (pos.0-1, pos.1),
            Dir::E  => (pos.0+1, pos.1),
        }
    }
}

pub struct Grid {
    pub grid: HashSet<(isize, isize)>,
}

impl Grid {
    pub fn new(cont: &parser::Content) -> Self {
        let mut grid: HashSet<(isize, isize)> = HashSet::new(); // (col, row) => Dir
        let mut row = 0;
        let mut col = 0;
        cont.content.lines().for_each(|l| {
            col = 0;
            l.chars().for_each(|c| {
                if c == '#' {
                    grid.insert((col, row));
                }
                col += 1;
            });
            row += 1;
        });
        Self { grid }
    }

    fn any_close(&self, pos: &(isize, isize), dirs: &Vec<Dir>) -> bool {
        for d in dirs { // Check if there are any positions filled nearby
            if d.get_positions(pos).iter()
                .any(|p| self.grid.contains(p)) {
                    return true
                }
        }
        return false
    }

    fn propose_single_elf(&self, pos: &(isize, isize), dirs: &Vec<Dir>) -> Option<(isize, isize)> {
        if !self.any_close(pos, dirs) { return None } // If we are alone in this 3x3 area
        for d in dirs {
            if let Some(x) = d.get_positions(pos).iter()
                .all(|p| !self.grid.contains(p))
                .then(|| d.go(pos)) {
                    return Some(x)
                }
        }
        return None
    }

    fn propose(&self, dirs: &Vec<Dir>) -> HashMap<(isize, isize), ((isize, isize), isize)> {
        let mut prop: HashMap<(isize, isize), ((isize, isize), isize)> = HashMap::new();
        self.grid.iter().for_each(|p| {
            if let Some(x) = self.propose_single_elf(p, &dirs) {
                match prop.get_mut(&x) {
                    None    => { prop.insert(x, (*p, 1)); },
                    Some(y) => { y.1 += 1; },
                }
            }
        });
        prop
    }

    fn step(&mut self, proposal: &HashMap<(isize, isize), ((isize, isize), isize)>) -> bool {
        let mut changed = false;
        proposal.iter().for_each(|(&new_p, val)| {
            if val.1 == 1 {
                changed = true;
                self.grid.remove(&val.0);
                self.grid.insert(new_p);
            }
        });
        changed
    }

    pub fn stationary(&mut self, dirs: &mut Vec<Dir>, n_max: isize) -> (isize, isize) {
        let mut proposal: HashMap<(isize, isize), ((isize, isize), isize)>;
        let mut changed = true;
        let mut n = 0;
        while changed && n < n_max {
            proposal = self.propose(&dirs);
            changed = self.step(&proposal);
            dirs.rotate_left(1);
            n += 1;
        }
        let x_diff = (self.grid.iter().map(|val| val.0).min().unwrap()
                      - self.grid.iter().map(|val| val.0).max().unwrap())
                    .abs()
                    +1;
        let y_diff = (self.grid.iter().map(|val| val.1).min().unwrap()
                      - self.grid.iter().map(|val| val.1).max().unwrap())
                    .abs()
                    +1;
        (x_diff * y_diff - self.grid.len() as isize, n)
    }

    pub fn visualise(&self) {
        let xmin = self.grid.iter().fold(isize::MAX, |acc, val| if val.0<acc { return val.0} else {return acc});
        let xmax = self.grid.iter().fold(isize::MIN, |acc, val| if val.0>acc { return val.0} else {return acc});
        let ymin = self.grid.iter().fold(isize::MAX, |acc, val| if val.1<acc { return val.1} else {return acc});
        let ymax = self.grid.iter().fold(isize::MIN, |acc, val| if val.1>acc { return val.1} else {return acc});
        let mut board = vec![vec!['.';(xmax+1-xmin)as usize];(ymax+1-ymin) as usize];
        for p in &self.grid {
            board[(p.1-ymin) as usize][(p.0-xmin) as usize] = '#';
        }
        for l in board {
            println!("{}", l.iter().collect::<String>());
        }
    }

}
