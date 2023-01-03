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
    pub x: T,
    pub y: T,
}

impl Point<isize> {
    // Return a vector of Points constructing a straight line in between self and other
    pub fn in_between(&self, other: &Point<isize>) -> Vec<Point<isize>> {
        let mut res: Vec<Point<isize>> = Vec::new();
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

    // Drop a point by x, y, return the new point
    pub fn drop(&self, x: isize, y: isize) -> Point<isize> {
        Point { x: self.x + x, y: self.y + y }
    }

    // Drop a point by x, y, in place
    pub fn drop_in_place(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }
}

pub struct Wall {
    pub paths: HashMap<Point<isize>, Tile>,
    pub lowest: isize,
}

impl Wall {
    pub fn new(c: &parser::Content) -> Self {
        let mut map: HashMap<Point<isize>, Tile> = HashMap::new();
        c.content.lines().for_each(|x| {
            let mut path: Vec<Point<isize>> = Vec::new();
            x.split(" -> ").for_each(|y| {
                let p: Vec<&str> = y.split(',').collect();
                path.push(Point {
                    x: p[0].parse::<isize>().unwrap(), 
                    y: p[1].parse::<isize>().unwrap()
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
    pub fn apply_rules(&mut self, sand: &mut Point<isize>) -> bool {
        if self.lowest + 1 == sand.y { //
            self.paths.insert(sand.clone(), Tile::Sand);
            return true
        } else if !self.paths.contains_key(&sand.drop(0, 1)) { // Down not blocked
            sand.drop_in_place(0, 1);
            return false
        } else if !self.paths.contains_key(&sand.drop(-1, 1)) { // Down left not blocked
            sand.drop_in_place(-1, 1);
            return false
        } else if !self.paths.contains_key(&sand.drop(1, 1)) { // Down right not blocked
            sand.drop_in_place(1, 1);
            return false
        } else { // Blocked
            self.paths.insert(sand.clone(), Tile::Sand);
            return true
        }
    }

    pub fn flow_sand(&mut self) {
        let mut prev_path = vec![ Point { x: 500, y: 0 } ];
        let mut sand: Point<isize>;
        'outer: loop {
            sand = prev_path.pop().unwrap();
            while sand.y < self.lowest {
                prev_path.push(sand.clone());
                if self.apply_rules(&mut sand) { 
                    break;
                }
            }

            if sand.y >= self.lowest {
                break 'outer;
            } else {
                prev_path.pop();
            }
        }
    }

    pub fn flow_sand_with_floor(&mut self) {
        let mut prev_path = vec![ Point { x: 500, y: 0 } ];
        let mut sand: Point<isize>;
        'outer: loop {
            sand = prev_path.pop().unwrap();
            while sand.y < self.lowest + 2 {
                prev_path.push(sand.clone());
                if self.apply_rules(&mut sand) { 
                    break;
                }
            }

            if sand.y == 0 {
                break 'outer;
            } else {
                prev_path.pop();
            }
        }
    }
}
