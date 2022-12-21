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

#[derive(Debug, Clone)]
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
        c.content.chars()
            .map(|x| if x == '<' { Jet::L } else if x == '>' { Jet::R } else { panic!("It literally took me 12 hours to find this error") })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone,Copy)]
struct Rock {
    r: u32,
    h: u8,
}

const ROCKS: [Rock; 5] = [
    Rock {
        r: 0x0000001E,
        h: 1,
    },
    Rock {
        r: 0x00081C08,
        h: 3,
    },
    Rock {
        r: 0x0004041C,
        h: 3,
    },
    Rock {
        r: 0x10101010,
        h: 4,
    },
    Rock {
        r: 0x00001818,
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

#[derive(Clone)]
struct Shaft<'a> {
    shaft: [u8; ROW_BUF_SIZE],
    height: usize,
    rocks: CyclicIndexable<'a, Rock>,
    jets: CyclicIndexable<'a, Jet>,
}

impl<'a> Shaft<'a> {
    fn new(rocks: &'a [Rock], jets: &'a [Jet]) -> Self {
        Self {
            shaft: [0; ROW_BUF_SIZE],
            height: 0,
            rocks: CyclicIndexable::new(rocks),
            jets: CyclicIndexable::new(jets)
        }
    }

    fn feasible(&self, rock: &Rock, row: usize) -> bool {
        rock.r & self.shaft_config(row) == 0
    }


    fn shaft_config(&self, row: usize) -> u32 {
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

    fn add_rock(&mut self, rock: &Rock, row: usize) -> usize {
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
        row as usize + rock.h as usize
    }

    fn clear_4_rows(&mut self, row: usize) {
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

    fn drop_rock(&mut self) -> usize {
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

    #[allow(dead_code)]
    fn print(&self, start_row: usize, end_row: usize) {
        for row in (start_row..=end_row).rev() {
            for col in (0..7).rev() {
                let pixel = if self.shaft[row] & (1 << col) == 0 {
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

pub fn simulate(jets: &Vec<Jet>, n_rocks: isize) -> usize {
    let mut shaft = Shaft::new(&ROCKS, jets);

    // Check how many rocks we've passed through for the specific [rock_idx][jet_idx] configuration 
    let mut jet_rock_after = vec![vec![isize::MIN; jets.len()]; ROCKS.len()];

    // For each rock
    for n in 0..n_rocks {
        let rock_idx = shaft.rocks.idx;
        let jet_idx = shaft.jets.idx;

        // If this configuration hasn't been tried
        if jet_rock_after[rock_idx][jet_idx] < 0 {

            // Set the value to the number of rocks we've dropped
            jet_rock_after[rock_idx][jet_idx] = n;

        // If it has been iterated
        } else {

            // create a new shaft and iterate that many rocks
            let mut shaft_new = Shaft::new(&ROCKS, jets);
            let n_new         = jet_rock_after[rock_idx][jet_idx];
            for _ in 0..n_new {
                shaft_new.drop_rock();
            }

            // check the difference in height between the two shafts and also how many more rocks
            // we've iterated in one compared to the other
            let height_diff = shaft.height - shaft_new.height;
            let n_diff      = n - n_new;

            // Make sure that they are equivalent for each stone we drop in both of them (if they
            // are we've found a repeating pattern)
            let mut equal = true;
            let mut shaft_copy = shaft.clone(); // clone so we don't iterate this shaft again
            for _ in 0..n_diff {
                if shaft_copy.drop_rock() != shaft_new.drop_rock() + height_diff {
                    equal = false;
                }
            }

            // if they are equal, we are done
            if equal {
                // Calculated how many more rocks I need to iterate in this period
                let n_rem = (n_rocks - n_new) % n_diff;
                // Calculate how many more periods we have
                let n_per = (n_rocks - n_new) / n_diff;
                for _ in 0..n_rem {
                    shaft.drop_rock();
                }
                return shaft.height + height_diff*(n_per as usize - 1);
            } 
        }

        // drop another rock
        shaft.drop_rock();
    }

    return shaft.height
}
