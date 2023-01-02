use std::collections::HashSet;
use parser;

const MAX_ROWS: usize = 32;

pub struct Grid {
    walls: [u128; MAX_ROWS],
    bliz_n: [u128; MAX_ROWS],
    bliz_s: [u128; MAX_ROWS],
    bliz_w: [u128; MAX_ROWS],
    bliz_e: [u128; MAX_ROWS],
    width: usize,
    height: usize,
    particles: [u128; MAX_ROWS],
}


impl Grid {
    pub fn new(cont: &parser::Content) -> Self {
        let mut walls  = [0u128; MAX_ROWS];
        let mut bliz_n = [0u128; MAX_ROWS];
        let mut bliz_s = [0u128; MAX_ROWS];
        let mut bliz_w = [0u128; MAX_ROWS];
        let mut bliz_e = [0u128; MAX_ROWS];
        let mut particles   = [0u128; MAX_ROWS];

        let width  = cont.content.lines().next().unwrap().len();
        let height = cont.content.lines().count();

        cont.content.lines().enumerate().for_each(|(row,l)| {
            l.chars().enumerate().for_each(|(col,c)| {
                match c {
                    '>' => bliz_e[row] |= 1 << (width - col - 1),
                    'v' => bliz_s[row] |= 1 << (width - col - 1),
                    '<' => bliz_w[row] |= 1 << (width - col - 1),
                    '^' => bliz_n[row] |= 1 << (width - col - 1),
                    '#' => walls[row]  |= 1 << (width - col - 1),
                    '.' => (),
                    _   => unreachable!(),
                };
            });
        });

        // particles are here positions that are slowly traversed through the labyrinth of blizzards
        particles[0] |= 1 << (width - 2); // start block

        Self { walls, bliz_n, bliz_s, bliz_w, bliz_e, width, height, particles }
    }

    #[allow(dead_code)]
	pub fn print(&self) {
		for r in 0..self.height {
			for c in (0..self.width).rev() {
                if (self.walls[r] & (1 << c)) != 0 {
                    print!("#");
                } else if (self.particles[r] & (1 << c)) != 0 {
                    print!("E");
                } else {
                    let n = self.bliz_n[r] & (1 << c);
                    let s = self.bliz_s[r] & (1 << c);
                    let w = self.bliz_w[r] & (1 << c);
                    let e = self.bliz_e[r] & (1 << c);
                    let num = n.count_ones() + s.count_ones() + w.count_ones() + e.count_ones();
                    match num {
                        0 => print!("."),
                        _ if num > 1 => print!("{}", num),
                        _ if n > 0 => print!("^"),
                        _ if s > 0 => print!("v"),
                        _ if w > 0 => print!("<"),
                        _ if e > 0 => print!(">"),
                        _ => unreachable!(),
                    }
                }
			}
			println!();
		}
		println!();
	}

    fn has_finished(&self) -> bool {
        (self.particles[self.height-1] & (1 << 1)) != 0
    }

    fn has_returned(&self) -> bool {
        (self.particles[0] & (1 << (self.width - 2))) != 0
    }

    fn blow_bliz(&mut self) {
        self.bliz_n[1..(self.height - 1)].rotate_left(1); // blow north blizs
        self.bliz_s[1..(self.height - 1)].rotate_right(1); // blow south blizs
        for r in 1..self.height-1 {
            // blow west blizs
            let bliz_w = self.bliz_w[r] << 1;
            self.bliz_w[r] = if bliz_w & self.walls[r] != 0 {
                bliz_w | (1 << 1)
            } else {
                bliz_w
            };
            // blow east blizs
            let bliz_e = self.bliz_e[r] >> 1;
            self.bliz_e[r] = if bliz_e & self.walls[r] != 0 {
                bliz_e | (1 << self.width-2)
            } else {
                bliz_e
            };
        }
    }

    pub fn step(&mut self) {
        self.blow_bliz(); // move all the blizzards in the room

        // move all the particles (positions) that are permissible, i.e., mark the positions of
        // standing still, moving N, S, W, E, and then remove the ones that collide with a blizzard
        let mut above = 0;
        for r in 0..self.height {                                                   // have to start from 0 to be able to reach the start (in exercise B)
            let current = self.particles[r];                                        // store all the particles on this level before evolving the particles
            self.particles[r] = current | above | (current << 1) | (current >> 1);  // adding particles:
                                                                                    //      standing still,
                                                                                    //      particles coming from the north,
                                                                                    //      particles going west, and
                                                                                    //      particles going east
            if r+1 < self.height {
                self.particles[r] |= self.particles[r+1];                           // adding particles: 
                                                                                    //      particles coming from the south (if we have enough rows)
            }
            above = current;                                                        // Set the "above" particles to the ones we had before change
                                                                                    // (because for the next row they can be "coming from the north")

            let blizzards = self.walls[r]                                           // Blizzards and walls 
                | self.bliz_n[r]
                | self.bliz_s[r]
                | self.bliz_w[r]
                | self.bliz_e[r];
            self.particles[r] &= !blizzards;                                        // Set all positions that are not occupied by blizzards or walls as possible particles
        }
    }

    pub fn travel(&mut self) -> usize {
        let mut time = 0;
        while !self.has_finished() {
            self.step();
            time += 1;
        }
        time
    }

    pub fn travel_twice(&mut self) -> usize {
        let mut time = 0;
        while !self.has_finished() {
            self.step();
            time += 1;
        }

        // Remove all particles excpet for the "finished" one
        self.particles.fill(0u128);
        self.particles[self.height-1] |= 1 << 1;

        while !self.has_returned() {
            self.step();
            time += 1;
        }

        // Remove all particles excpet for the "starting" one
        self.particles.fill(0u128);
        self.particles[0] |= 1 << (self.width - 2);

        while !self.has_finished() {
            self.step();
            time += 1;
        }

        time
    }
}



