use parser;

// Wall:
// 10000000(1)
// 10000000(1)
// 10000000(1)
// 10000000(1)
//
// Horizontal Line:
// 00000000
// 00000000
// 00000000
// 00011110
//
// Cross:
// 00000000
// 00001000
// 00011100
// 00001000
//
// L:
// 00000000
// 00000100
// 00000100
// 00011100
//
// Vertical Line:
// 00010000
// 00010000
// 00010000
// 00010000
//
// Cube:
// 00000000
// 00000000
// 00011000
// 00011000

struct CyclicIndexable<'a,T> {
    vec: &'a [T],
    idx: usize,
    len: usize,
}

impl<'a,T> CyclicIndexable<'a,T> {
    fn new(vec: &'a [T]) -> Self {
        CyclicIndexable{vec, idx: 0, len: vec.len()}
    }
}

impl<'a,T> Iterator for CyclicIndexable<'a,T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let el = self.vec[self.idx];
        self.idx = (self.idx + 1) % self.len;
        return Some(el);
    }
}

#[derive(Clone,Copy)]
pub enum Jet {
    L,
    R,
}

impl Jet {
    pub fn new(c: &parser::Content) -> Vec<Self> {
        c.content.chars().map(|x| if x == '<' { Jet::L } else { Jet::R }).collect::<Vec<_>>()
    }
}

#[derive(Clone,Copy)]
pub struct Rock {
    r: u32,
    w: u8,
    h: u8,
}

const ROCKS: [Rock; 5] = [
    Rock {
        r: 0x0000001E,
        w: 4,
        h: 1,
    },
    Rock {
        r: 0x00081C08,
        w: 3,
        h: 3,
    },
    Rock {
        r: 0x0004041C,
        w: 3,
        h: 3,
    },
    Rock {
        r: 0x10101010,
        w: 1,
        h: 4,
    },
    Rock {
        r: 0x00001818,
        w: 2,
        h: 2,
    }
];

impl Rock {
    fn shift(&mut self, j: &Jet, shaft_config: u32) {
        let np = match j {
            Jet::L => {
                if self.r & 0x40404040 == 0 { // block before left wall
                    self.r << 1
                } else {
                    return
                }
            },
            Jet::R => {
                if self.r & 0x01010101 == 0 { // block before right wall
                    self.r >> 1
                } else {
                    return
                }
            }
        };

        if np & shaft_config == 0 {
            self.r = np;
        }
    }

    fn as_bytes(&self) -> [u8; 4] {
        self.r.to_le_bytes()
    }
}

const ROW_BUF_SIZE: usize = 1_024;

struct Shaft<'a> {
    shaft: [u8; ROW_BUF_SIZE],
    height: isize,
    rocks: CyclicIndexable<'a, Rock>,
    jets: CyclicIndexable<'a, Jet>,
}

impl<'a> Shaft<'a> {
    pub fn new(rocks: &'a [Rock], jets: &'a [Jet]) -> Self {
        Self {
            shaft: [0; ROW_BUF_SIZE],
            height: 0,
            rocks: CyclicIndexable::new(rocks),
            jets: CyclicIndexable::new(jets)
        }
    }

    fn feasible(&self, rock: &Rock, row: isize) -> bool {
        if row < 0 { return false }
        rock.r & self.shaft_config(row) == 0
    }


    fn shaft_config(&self, row: isize) -> u32 {
        let row = row as usize;
        let r3 = (row + 3) % ROW_BUF_SIZE;
        let r2 = (row + 2) % ROW_BUF_SIZE;
        let r1 = (row + 1) % ROW_BUF_SIZE;
        let r0 = (row + 0) % ROW_BUF_SIZE;
        let mut res: u32 = self.shaft[r3] as u32;
        res = (res << 8) | self.shaft[r2] as u32;
        res = (res << 8) | self.shaft[r1] as u32;
        (res << 8) | self.shaft[r0] as u32
    }

    fn add_rock(&mut self, rock: &Rock, row: isize) -> isize {
        let row = row as usize;
        let r3 = (row + 3) % ROW_BUF_SIZE;
        let r2 = (row + 2) % ROW_BUF_SIZE;
        let r1 = (row + 1) % ROW_BUF_SIZE;
        let r0 = (row + 0) % ROW_BUF_SIZE;
        let r  = rock.as_bytes();
        self.shaft[r3] |= r[3];
        self.shaft[r2] |= r[2];
        self.shaft[r1] |= r[1];
        self.shaft[r0] |= r[0];
        row as isize + rock.h as isize
    }

    fn clear_4_rows(&mut self, row: isize) {
        let row = row as usize;
        let r3 = (row + 3) % ROW_BUF_SIZE;
        let r2 = (row + 2) % ROW_BUF_SIZE;
        let r1 = (row + 1) % ROW_BUF_SIZE;
        let r0 = (row + 0) % ROW_BUF_SIZE;

        self.shaft[r3] = 0;
        self.shaft[r2] = 0;
        self.shaft[r1] = 0;
        self.shaft[r0] = 0;
    }

    fn drop_rock(&mut self) -> isize {
        let mut row  = self.height + 3;
        let mut rock = self.rocks.next().unwrap();

        self.clear_4_rows(row);

        loop {
            let jet = self.jets.next().unwrap();

            rock.shift(&jet, self.shaft_config(row));

            if row > 0 && self.feasible(&rock, row-1) {
                row -= 1;
            } else {
                let top_row_added = self.add_rock(&rock, row);
                self.height = self.height.max(top_row_added);
                break;
            }
        }

        return row
    }

    fn print(&self, start_row: usize, end_row: usize) {
        for row in (start_row..=end_row).rev() {
            for col in (0..7).rev() {
                let r = row as usize % ROW_BUF_SIZE;
                let row_mask = 1 << col;
                let pixel = if self.shaft[r] & row_mask == 0 {
                    "."
                } else {
                    "#"
                };
                print!("{}", pixel);
            }
            println!();
        }
    }
}

pub fn simulate(jets: &Vec<Jet>, n_rocks: usize) -> isize {
    let mut shaft = Shaft::new(&ROCKS, &jets);
    let mut jet_idx = 0;
    for _ in 0..n_rocks {
        shaft.drop_rock();
    }
    shaft.height
}

//pub fn simulate_periodic(jets: &Vec<Jet>, n_rocks: usize) -> usize {
//    let mut shaft = Shaft::new();
//    let mut jet_idx = 0;
//    let mut ele_idx = 0;
//
//    let mut shape_move_seen_after = vec![vec![-1i64; jets.len()]; 5];
//
//    for simulated_rocks in 0..n_rocks {
//        if shape_move_seen_after[ele_idx][jet_idx] < 0 {
//            shape_move_seen_after[ele_idx][jet_idx] = simulated_rocks;
//        } else {
//            let mut ref_shaft = Shaft::new();
//            let n_rocks_transient = shape_move_seen_after[ele_idx][jet_idx];
//            for n in 0..n_rocks_transient {
//                (_, jet_idx) = ref_shaft.drop_rock(jets, ele_idx, jet_idx);
//                ele_idx = n;
//            }
//            let height_period = shaft.height - ref_shaft.height;
//            let n_rocks_period = simulated_rocks - n_rocks_transient;
//
//            let mut equal = true;
//            for n in 0..n_rocks_period {
//                let (row,col) = chamber.simulate_next_rock();
//                let (row_ref,col_ref) = ref_chamber.simulate_next_rock();
//
//                if !(row == row_ref + height_period && col == col_ref) {
//                    equal = false;
//                }
//            }
//
//            chamber = ref_chamber;
//
//            if equal {
//                let remainder = (n_rocks - n_rocks_transient) % n_rocks_period;
//                let n_periods = (n_rocks - n_rocks_transient) / n_rocks_period;
//                for _ in 0..remainder {
//                    chamber.simulate_next_rock();
//                }
//                return chamber.height + height_period*(n_periods - 1)
//            }
//        }
//
//        chamber.simulate_next_rock();
//    }
//    return chamber.height;
//}
