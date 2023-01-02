use std::collections::HashSet;
use parser;

#[derive(Eq, PartialEq, Hash)]
#[derive(Clone, Copy)]
pub struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord {
    fn new(line: &str) -> Self {
        let v = line.split(',').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
        Self { x: v[0], y: v[1], z: v[2] }
    }

    pub fn neighbours(&self) -> CoordIterator {
        CoordIterator::new(*self)
    }
}

pub struct CoordIterator {
    coord: Coord,
    idx: usize,
}

impl CoordIterator {
    pub fn new(coord: Coord) -> Self {
        Self { coord, idx: 0 }
    }
}

impl Iterator for CoordIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Coord> {
        let CoordIterator { coord, idx } = self;
        let Coord { x, y, z } = coord;

        let res = match idx {
            0 => Some( Coord { x: *x-1, y: *y,   z: *z } ),
            1 => Some( Coord { x: *x+1, y: *y,   z: *z } ),
            2 => Some( Coord { x: *x,   y: *y-1, z: *z } ),
            3 => Some( Coord { x: *x,   y: *y+1, z: *z } ),
            4 => Some( Coord { x: *x,   y: *y,   z: *z-1 } ),
            5 => Some( Coord { x: *x,   y: *y,   z: *z+1 } ),
            _ => None,
        };

        *idx += 1;
        res
    }
}

pub struct Grid {
    grid: HashSet<Coord>,
}

impl Grid {
    pub fn new(cont: &parser::Content) -> Self {
        let mut grid: HashSet<Coord> = cont.content.lines()
            .map(|l| Coord::new(l))
            .collect();
        Self { grid }
    }

    pub fn total_area(&self) -> isize {
        self.grid.iter()
            .map(|crd| crd.neighbours()
                 .map(|nb| if !self.grid.contains(&nb) { 1 } else { 0 })
                 .sum::<isize>())
            .sum()
    }

    pub fn visible_area(&mut self, tot_area: Option<isize>) -> isize {
        let tot_area = match tot_area {
            Some(x) => x,
            None    => self.total_area(),
        };

        // Create bounding box
        let min_x = self.grid.iter().map(|crd| crd.x).min().unwrap() - 2;
        let min_y = self.grid.iter().map(|crd| crd.y).min().unwrap() - 2;
        let min_z = self.grid.iter().map(|crd| crd.z).min().unwrap() - 2;
        let max_x = self.grid.iter().map(|crd| crd.x).max().unwrap() + 2;
        let max_y = self.grid.iter().map(|crd| crd.y).max().unwrap() + 2;
        let max_z = self.grid.iter().map(|crd| crd.z).max().unwrap() + 2;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                self.grid.insert(Coord { x, y, z: min_z });
                self.grid.insert(Coord { x, y, z: max_z });
            }
        }
        for x in min_x..=max_x {
            for z in min_z..=max_z {
                self.grid.insert(Coord { x, y: min_y, z });
                self.grid.insert(Coord { x, y: max_y, z });
            }
        }
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                self.grid.insert(Coord { x: min_x, y, z });
                self.grid.insert(Coord { x: max_x, y, z });
            }
        }

        // Flood-fill algorithm
        let mut queue = vec![Coord { x: min_x + 1, y: min_y + 1, z: min_z + 1 }];
        while let Some(crd) = queue.pop() { // Pop element from queue
            if self.grid.insert(crd) { // If it doesn't already exist in the grid
                crd.neighbours().for_each(|nb| {
                    if !self.grid.contains(&nb) {
                        queue.push(nb);
                    }
                });
            }
        }
        
        // Extract total area of outer bounding box
        let bounding_area = 2 * (
            (max_x - min_x + 1) * (max_y - min_y + 1)
            + (max_x - min_x + 1) * (max_z - min_z + 1)
            + (max_z - min_z + 1) * (max_y - min_y + 1));
        // The difference between the new total area and the bounding box's area is the internal area
        let internal_area = self.total_area() - bounding_area;
        // Remove the internal area from the old total area
        tot_area - internal_area
    }
}

