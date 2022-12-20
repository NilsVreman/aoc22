use parser;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Valve {
    flow: usize,
    paths: Vec<usize>,
}

pub struct State {
    curr: usize,
    t: usize,
    score: usize,
    opened: HashSet<usize>,
}

pub struct ElephantState {
    curr: (usize, usize),
    t: (usize, usize),
    score: usize,
    opened: HashSet<usize>,
}

impl State {
    pub fn new(score: usize, time: usize) -> Self {
        Self {
            curr: to_usize("AA"),
            score: score,
            t: time,
            opened: HashSet::new(),
        }
    }
}

impl ElephantState {
    pub fn new(score: usize, time: usize) -> Self {
        Self {
            curr: (to_usize("AA"), to_usize("AA")),
            score: score,
            t: (time, time),
            opened: HashSet::new(),
        }
    }
}

impl Valve {
    pub fn new(s: &Vec<&str>) -> Self {
        Self {
            flow: s[1].parse::<usize>().unwrap(),
            paths: s[5].split(", ").map(|x| to_usize(x)).collect::<Vec<usize>>(),
        } 
    }

    pub fn pressure_release(&self, t_left: usize) -> usize {
        self.flow*t_left
    }
}

pub struct Grid {
    map: HashMap<usize, Valve>,
    dist: HashMap<(usize, usize), usize>,
}

impl Grid {
    pub fn new(c: &parser::Content) -> Self {
        let mut map: HashMap<usize, Valve> = HashMap::new();
        let ps = parser::Parser::build("Valve {} has flow rate={}; {} {} to {} {}", "{}");
        c.content.lines().for_each(|l| {
            let s = ps.parse(&l).unwrap();
            map.insert(to_usize(&s[0]), Valve::new(&s));
        });

        let mut unvisited: HashSet<usize> = HashSet::new();
        map.iter().filter(|(_, v)| v.flow != 0).for_each(|(&k, _)| { unvisited.insert(k); });
        unvisited.insert(to_usize("AA"));

        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        for &k1 in &unvisited {
            for &k2 in &unvisited {
                dist.insert((k1, k2), Grid::shortest_dist(&map, k1, k2));
            }
        }
        Self { map, dist }
    }

    fn shortest_dist(map: &HashMap<usize, Valve>, current: usize, target: usize) -> usize {
        let mut unvisited: Vec<usize> = Vec::new();
        let mut dist: HashMap<usize, usize> = HashMap::new();
        map.iter().for_each(|(&k, _)| {
            dist.insert(k, usize::MAX);
            unvisited.push(k);
        });
        dist.insert(current, 0);
        while !unvisited.is_empty() {
            let mut u: usize = 0;
            let mut min = usize::MAX;
            unvisited.iter().for_each(|x| if dist[x] < min { u = *x; min = dist[x]; });
            if u == target {
                return dist[&u];
            }
            let idx = unvisited.iter().position(|x| x == &u).unwrap();
            unvisited.remove(idx);

            for n in map[&u].paths.clone() {
                let d = dist[&u] + 1;
                dist.insert(n, d.min(dist[&n]));
            }
        }
        return dist[&target]
    }

    pub fn max_pressure_release(&self, state: &State) -> usize {
        self.dfs(&state)
    }

    fn dfs(&self, state: &State) -> usize {
        let mut best = state.score;
        for &n in self.map.keys() {
            if self.map[&n].flow > 0 && !state.opened.contains(&n) {
                if let Some(dur) = self.dist.get(&(state.curr, n)) {
                    let dur = dur + 1;
                    if state.t > dur {
                        let mut opened = state.opened.clone();
                        opened.insert(n);
                        best = self.dfs(&State{
                            score: state.score + self.map[&n].pressure_release(state.t - dur),
                            t: state.t - dur,
                            curr: n,
                            opened,
                        }).max(best);
                    }
                }
            }
        }
        best
    }

    pub fn max_pressure_release_w_elephant(&self, state: &ElephantState) -> usize {
        self.dfse(&state)
    }

    fn dfse(&self, state: &ElephantState) -> usize {
        let mut best = state.score;
        for &n in self.map.keys() {
            if self.map[&n].flow > 0 && !state.opened.contains(&n) {
                if state.t.0 < state.t.1 {
                    if let Some(dur) = self.dist.get(&(state.curr.1, n)) {
                        let dur = dur + 1;
                        if state.t.1 > dur {
                            let mut opened = state.opened.clone();
                            opened.insert(n);
                            best = self.dfse(&ElephantState{
                                score: state.score + self.map[&n].pressure_release(state.t.1 - dur),
                                t: (state.t.0, state.t.1 - dur),
                                curr: (state.curr.0, n),
                                opened,
                            }).max(best);
                        }
                    }
                } else {
                    if let Some(dur) = self.dist.get(&(state.curr.0, n)) {
                        let dur = dur + 1;
                        if state.t.0 > dur {
                            let mut opened = state.opened.clone();
                            opened.insert(n);
                            best = self.dfse(&ElephantState{
                                score: state.score + self.map[&n].pressure_release(state.t.0 - dur),
                                t: (state.t.0 - dur, state.t.1),
                                curr: (n, state.curr.1),
                                opened,
                            }).max(best);
                        }
                    }
                }
            }
        }
        best
    }

}

pub fn to_usize(s: &str) -> usize {
    let mut out: usize = 0;
    s.as_bytes()
        .iter()
        .for_each(|&x| {
            out <<= 8;
            out |= x as usize;
        });
    out
}
