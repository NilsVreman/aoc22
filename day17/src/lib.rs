use parser;

const WIDTH: usize = 7;
const NUM_ELE: usize = 2022;
const HEIGHT: usize = 4*NUM_ELE+10;

pub enum Jet {
    L,
    R,
}

impl Jet {
    pub fn new(c: &parser::Content) -> Vec<Self> {
        c.content.chars().map(|x| if x == '<' { Jet::L } else { Jet::R }).collect::<Vec<Jet>>()
    }
}

enum Rock {
    HLine,
    Cross,
    L,
    VLine,
    Square,
}

impl Rock {
    fn get_shape(ele: usize) -> Rock {
        match ele % 5 {
            0 => Rock::HLine,
            1 => Rock::Cross,
            2 => Rock::L,
            3 => Rock::VLine,
            4 => Rock::Square,
            _ => panic!("Not sure why this is needed"),
        }
    }

    pub fn get_height(&self) -> usize {
        match self {
            Rock::HLine => 1,
            Rock::Cross => 3,
            Rock::L => 3,
            Rock::VLine => 4,
            Rock::Square => 2,
        }
    }
}

pub struct Shaft {
    pub shaft: [[bool; WIDTH]; HEIGHT],
    pub top: usize,
}

impl Shaft {
    pub fn new() -> Self {
        Self { shaft: [[false; WIDTH]; HEIGHT], top: 0 }
    }

    fn feasible(&self, shape: &Rock, x: &usize, y: &usize) -> bool {
        match shape {
            Rock::HLine => {
                *x <= 3
                    && !self.shaft[*y][*x]
                    && !self.shaft[*y][*x+1]
                    && !self.shaft[*y][*x+2]
                    && !self.shaft[*y][*x+3]
            },
            Rock::Cross => {
                *x <= 4
                    && !self.shaft[*y+1][*x]
                    && !self.shaft[*y+1][*x+1]
                    && !self.shaft[*y+1][*x+2]
                    && !self.shaft[*y][*x+1]
                    && !self.shaft[*y+2][*x+1]
            },
            Rock::L => {
                *x <= 4
                    && !self.shaft[*y][*x]
                    && !self.shaft[*y][*x+1]
                    && !self.shaft[*y][*x+2]
                    && !self.shaft[*y+1][*x+2]
                    && !self.shaft[*y+2][*x+2]
            },
            Rock::VLine => {
                *x <= 6
                    && !self.shaft[*y][*x]
                    && !self.shaft[*y+1][*x]
                    && !self.shaft[*y+2][*x]
                    && !self.shaft[*y+3][*x]
            },
            Rock::Square => {
                *x <= 5
                    && !self.shaft[*y][*x]
                    && !self.shaft[*y+1][*x]
                    && !self.shaft[*y][*x+1]
                    && !self.shaft[*y+1][*x+1]
            },
        }
    }

    fn feasible_shift(&self, shape: &Rock, j: &Jet, x: &usize, y: &usize) -> bool {
        match j {
            Jet::L => *x >= 1 && self.feasible(&shape, &(x-1), y),
            Jet::R => *x <= 6 && self.feasible(&shape, &(x+1), y),
        }
    }

    fn set(&mut self, shape: &Rock, x: &usize, y: &usize) {
        match shape {
            Rock::HLine => {
                self.shaft[*y][*x]      = true;
                self.shaft[*y][*x+1]    = true;
                self.shaft[*y][*x+2]    = true;
                self.shaft[*y][*x+3]    = true;
            },
            Rock::Cross => {
                self.shaft[*y+1][*x]    = true;
                self.shaft[*y+1][*x+1]  = true;
                self.shaft[*y+1][*x+2]  = true;
                self.shaft[*y][*x+1]    = true;
                self.shaft[*y+2][*x+1]  = true;
            },
            Rock::L => {
                self.shaft[*y][*x]      = true;
                self.shaft[*y][*x+1]    = true;
                self.shaft[*y][*x+2]    = true;
                self.shaft[*y+1][*x+2]  = true;
                self.shaft[*y+2][*x+2]  = true;
            },
            Rock::VLine => {
                self.shaft[*y][*x]      = true;
                self.shaft[*y+1][*x]    = true;
                self.shaft[*y+2][*x]    = true;
                self.shaft[*y+3][*x]    = true;
            },
            Rock::Square => {
                self.shaft[*y][*x]      = true;
                self.shaft[*y+1][*x]    = true;
                self.shaft[*y][*x+1]    = true;
                self.shaft[*y+1][*x+1]  = true;
            },
        }
    }

    fn shift_and_drop(&mut self, shape: &Rock, j: &Jet, x: &usize, y: &usize) -> (usize, usize) {
        let (mut res_x, mut res_y) = (*x as isize, *y as isize);
        if self.feasible_shift(&shape, &j, x, y) {
            res_x = res_x + match j {
                Jet::L => -1,
                Jet::R => 1,
            };
        }

        if res_y == 0 || !self.feasible(&shape, &(res_x as usize), &((res_y-1) as usize)) {
            self.set(shape, &(res_x as usize), &(res_y as usize));
            self.top = &(res_y as usize) + shape.get_height();
        } else {
            res_y -= 1;
        }
        return (res_x as usize, res_y as usize)
    }

    pub fn iterate(&mut self, jets: &Vec<Jet>) -> usize {
        let mut idx = 0;
        for ele in 0..NUM_ELE {
            let shape = Rock::get_shape(ele);
            let (mut x, mut y) = (2, self.top + 3);
            loop {
                let j = &jets[idx];
                idx += 1;
                if idx >= jets.len() {
                    idx = 0;
                }
                let (xn, yn) = self.shift_and_drop(&shape, &j, &x, &y);
                x = xn;
                if y != yn {
                    y = yn;
                } else {
                    break
                }
            }
        }
        self.top + 3
    }
}
