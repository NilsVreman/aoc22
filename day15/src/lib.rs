use parser;
use std::cmp;

#[derive(Eq, Hash, PartialEq)]
pub struct Sensor {
    pos: (isize, isize),
    dist: isize,
}

impl Sensor {
    pub fn is_inside(&self, other: (isize, isize), interval: (isize, isize)) -> bool {
        if other.0 < interval.0 || other.0 > interval.1 || other.1 < interval.0 || other.1 > interval.1 {
            return true
        }
        self.dist >= (self.pos.0-other.0).abs()+(self.pos.1-other.1).abs()
    }
}

pub fn new_list(c: &parser::Content) -> Vec<Sensor> {
    let ps = parser::Parser::build("Sensor at x={}, y={}: closest beacon is at x={}, y={}", "{}");
    c.content.lines().map(|l| {
        let v = ps.parse(l).unwrap(); // Panics if error
        let x1 = v[0].parse::<isize>().unwrap();
        let y1 = v[1].parse::<isize>().unwrap();
        let x2 = v[2].parse::<isize>().unwrap();
        let y2 = v[3].parse::<isize>().unwrap();
        let md = (x1-x2).abs()+(y1-y2).abs();
        Sensor { pos: (x1, y1), dist: md }
    }).collect::<Vec<Sensor>>()
}

fn merge_intervals(list: &mut Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    list.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut res: Vec<Vec<isize>> = Vec::new();
    res.push(list[0].clone());

    for i in 1..list.len() {
        let current = list[i].clone();
        let j = res.len() - 1;

        if current[0] >= res[j][0] && current[0] <= res[j][1] {
            res[j][1] = cmp::max(current[1], res[j][1]);
        } else {
            res.push(current);
        }
    }
    res
}

pub fn cant_contain_beacon(list: &Vec<Sensor>, row: isize) -> isize {
    let mut res: Vec<Vec<isize>> = Vec::new();
    list.iter().for_each(|x| {
        let xmd = x.dist - (x.pos.1-row).abs();
        if xmd > 0 {
            res.push(vec![x.pos.0-xmd, x.pos.0+xmd]);
        }
    });
    res = merge_intervals(&mut res);
    res.iter().map(|x| x[1]-x[0]+1).sum::<isize>()
}

pub fn can_contain_beacon(list: &Vec<Sensor>, size: isize) -> usize {
    list.iter()
        .find_map(|x| {
            ((x.pos.0 - x.dist - 1)..=x.pos.0)
                .zip(x.pos.1..=(x.pos.1 + x.dist + 1)) // Realise that if there is a beacon in the set it has to be
                                                       // at the boundary of one of the sensor ranges.
                .find_map(|p| { // Check if all the points on this sensors boundary are inside some
                                // other sensors range
                    list.iter()
                        .all(|s| !s.is_inside(p, (0, size)))
                        .then(|| p.0 * size + p.1 )
                })
        }).unwrap() as usize
}
