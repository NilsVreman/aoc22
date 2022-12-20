use parser;
use std::collections::HashMap;

pub enum Action {
    Move,
    Open(String),
}

#[derive(Debug)]
pub struct Valve {
    flow: isize,
    paths: Vec<String>,
}

impl Valve {
    pub fn pressure_release(&self, t_left: isize) -> isize {
        self.flow*t_left
    }
}

pub struct Grid {
    start: String,
    map: HashMap<String, Valve>,
}

impl Grid {
    pub fn new(c: &parser::Content) -> Self {
        let ps = parser::Parser::build("Valve {} has flow rate={}; {} {} to {} {}", "{}");
        let mut map: HashMap<String, Valve> = HashMap::new();
        let s = ps.parse(&c.content.lines().next().unwrap()).expect("Couldn't parse first line");
        let start = Valve {
            flow: s[1].parse::<isize>().unwrap(),
            paths: s[5].split(", ").map(|x| x.to_string()).collect::<Vec<String>>()
        };
        map.insert(s[0].to_string(), start);
        c.content.lines().skip(1).for_each(|l| {
            let s = ps.parse(l).expect("Couldn't parse line");
            let v = Valve {
                flow: s[1].parse::<isize>().unwrap(),
                paths: s[5].split(", ").map(|x| x.to_string()).collect::<Vec<String>>()
            };
            map.insert(s[0].to_string(), v);
        });
        Self { start: "AA".to_string(), map }
    }

    pub fn max_pressure_release(&self, t_max: isize) -> isize {
        let opened: Vec<String> = Vec::new();
        self.itr_max(&self.start, &"".to_string(), t_max, &opened)
    }

    fn itr_max(&self,
               current: &String,
               prev: &String,
               t_left: isize,
               opened: &Vec<String>)
        -> isize {
        if t_left <= 0 { return 0 }

        let mut best: isize = 0;

        if self.map[current].flow > 0 && !opened.contains(&current) {
            let pr_current = self.map[current].pressure_release(t_left-1);

            let mut c_opened = opened.clone();
            c_opened.push(current.clone());

            best = pr_current + self.map[current].paths.iter()
                .map(|x| self.itr_max(x, current, t_left-2, &c_opened) )
                .max()
                .unwrap();
        }
        best.max(self.map[current].paths.iter()
                 .map(|x| if x == prev { -1 } else { self.itr_max(x, current, t_left-1, &opened) })
                 .max()
                 .unwrap())
    }

    //pub fn max_pressure_release_w_ele(&self, t_max: isize) -> isize {
    //    let v: Vec<String> = Vec::new();
    //    self.itr_max(&self.start, &"".to_string(), t_max, &v)
    //}

    //fn itr_max(&self, 
    //           current_me: &String, 
    //           prev_me: &String,
    //           current_ele: &String,
    //           prev_ele: &String,
    //           t_left: isize,
    //           opened: &Vec<String>)
    //    -> isize {
    //    if t_left <= 0 { return 0 }

    //    let mut best: isize = 0;

    //    if (self.map[current_me].flow > 0, !opened.contains(&current_me), self.map[current_ele].flow > 0, !opened.contains(&current_ele)) {
    //        let pr_current = self.map[current].pressure_release(t_left-1);

    //        let mut c_opened = opened.clone();
    //        c_opened.push(current.clone());

    //        best = pr_current + self.map[current].paths.iter()
    //            .map(|x| self.itr_max(x, current, t_left-2, &c_opened) )
    //            .max()
    //            .unwrap();
    //    }
    //    best.max(self.map[current].paths.iter()
    //             .map(|x| if x == prev { -1 } else { self.itr_max(x, current, t_left-1, &opened) })
    //             .max()
    //             .unwrap())
    //}
}

