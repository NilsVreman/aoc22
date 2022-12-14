use parser;
use std::collections::HashMap;

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub enum Tile {
    Rock,
    Sand,
}

#[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    pub fn in_between(&self, other: &Point<i32>) -> Vec<Point<i32>> {
        let mut res: Vec<Point<i32>> = Vec::new();
        if self.x == other.x {
            for y in 0..=(self.y-other.y).abs() {
                res.push( Point {
                    x: self.x,
                    y: self.y-(self.y-other.y).signum()*y
                });
            }
        } else { // self.y == other.y (since they are straight lines)
            for x in 0..=(self.x-other.x).abs() {
                res.push( Point {
                    x: self.x-(self.x-other.x).signum()*x,
                    y: self.y,
                });
            }
        }
        res
    }

    pub fn fall(&self, x: i32, y: i32) -> Point<i32> {
        Point { x: self.x + x, y: self.y + y }
    }

    pub fn fall_in_place(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

pub struct Wall {
    pub paths: HashMap<Point<i32>, Tile>,
    pub lowest: i32,
}

impl Wall {
    pub fn new(c: &parser::Content) -> Self {
        let mut map: HashMap<Point<i32>, Tile> = HashMap::new();
        c.content.lines().for_each(|x| {
            let mut path: Vec<Point<i32>> = Vec::new();
            x.split(" -> ").for_each(|y| {
                let p: Vec<&str> = y.split(',').collect();
                path.push(Point {
                    x: p[0].parse::<i32>().unwrap(), 
                    y: p[1].parse::<i32>().unwrap()
                });
            }); // Get a list of all the path nodes
            path[..path.len()-1].iter().zip(path[1..].iter()) // Zip together path nodes connecting to one another
                .for_each(|(p1, p2)| { // For each of these pairs: Insert that path block into the map
                    p1.in_between(p2).iter()
                        .for_each(|p| {
                            map.insert(p.clone(), Tile::Rock);
                        })
                })
        });
        let lowest_level = map.keys().map(|p| p.y).max().expect("Should have received the lowest level");
        Self { paths: map, lowest: lowest_level }
    }

    // I know I could speed this up by not constructing a new Point every rule check.
    pub fn apply_rules(&mut self, sand: &mut Point<i32>) -> bool {
        if !self.paths.contains_key(&sand.fall(0, 1)) { // Down not blocked
            sand.fall_in_place(0, 1);
            return false
        } else if !self.paths.contains_key(&sand.fall(-1, 1)) { // Down left not blocked
            sand.fall_in_place(-1, 1);
            return false
        } else if !self.paths.contains_key(&sand.fall(1, 1)) { // Down right not blocked
            sand.fall_in_place(1, 1);
            return false
        } else { // Blocked
            self.paths.insert(sand.clone(), Tile::Sand);
            return true
        }
    }

    pub fn flow_sand(&mut self) -> bool {
        let mut sand = Point { x: 500, y: 0 };
        while sand.y < self.lowest {
            if self.apply_rules(&mut sand) {
                return true
            }
        }
        false
    }

    pub fn flow_sand_with_floor(&mut self) -> bool {
        let mut sand = Point { x: 500, y: 0 };
        let ground = self.lowest + 2;
        while !self.paths.contains_key(&sand) {
            if sand.y + 1 == ground { // If reached lowest
                self.paths.insert(sand, Tile::Sand);
                return true
            } 
            if self.apply_rules(&mut sand) {
                return true
            }
        }
        false
    }
}
