use std::collections::HashMap;
use parser;

fn str_to_u32(s: &str) -> u32 {
    if s.len() != 4 { return 0; }
    let mut res: u32 = 0;
    s.chars().for_each(|c| {
        res = (res << 8) | (c as u32);
    });
    res
}

enum Expr {
    Val(isize),
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
    Div(u32, u32),
}

impl Expr {
    pub fn new(s: &str) -> Self {
        if let Ok(x) = s.parse::<isize>() {
            Expr::Val(x)
        } else {
            let v = s.split(' ').collect::<Vec<_>>();
            match v[1] {
                "+" => Expr::Add(str_to_u32(v[0]), str_to_u32(v[2])),
                "-" => Expr::Sub(str_to_u32(v[0]), str_to_u32(v[2])),
                "*" => Expr::Mul(str_to_u32(v[0]), str_to_u32(v[2])),
                "/" => Expr::Div(str_to_u32(v[0]), str_to_u32(v[2])),
                _   => panic!("unknown expression"),
            }
        }
    }
}

pub struct Job {
    id: u32,
    expr: Expr,
}

impl Job {

    // Generated using str_to_u32
    const HUMN: u32 = 1752526190;

    pub fn new(line: &str) -> Self {
        let v = line.split(": ").collect::<Vec<_>>();
        Self { id: str_to_u32(v[0]), expr: Expr::new(v[1]) }
    }

    pub fn eval(&self, jobs: &HashMap<u32, Job>) -> isize {
        match self.expr {
            Expr::Val(x)      => x,
            Expr::Add(x, y)   => jobs[&x].eval(jobs) + jobs[&y].eval(jobs),
            Expr::Sub(x, y)   => jobs[&x].eval(jobs) - jobs[&y].eval(jobs),
            Expr::Mul(x, y)   => jobs[&x].eval(jobs) * jobs[&y].eval(jobs),
            Expr::Div(x, y)   => jobs[&x].eval(jobs) / jobs[&y].eval(jobs),
        }
    }

    fn humn_in_chain(&self, jobs: &HashMap<u32, Job>) -> bool {
        if self.id == Job::HUMN {
            true
        } else {
            match self.expr {
                Expr::Val(_)      => false,
                Expr::Add(x, y)   => jobs[&x].humn_in_chain(jobs) || jobs[&y].humn_in_chain(jobs),
                Expr::Sub(x, y)   => jobs[&x].humn_in_chain(jobs) || jobs[&y].humn_in_chain(jobs),
                Expr::Mul(x, y)   => jobs[&x].humn_in_chain(jobs) || jobs[&y].humn_in_chain(jobs),
                Expr::Div(x, y)   => jobs[&x].humn_in_chain(jobs) || jobs[&y].humn_in_chain(jobs),
            }
        }
    }

    fn eval_needed(&self, jobs: &HashMap<u32, Job>, val_needed: isize) -> isize {
        if self.id == Job::HUMN {
            val_needed
        } else {
            match self.expr {
                Expr::Add(x, y)   => {
                    if jobs[&x].humn_in_chain(jobs) {
                        jobs[&x].eval_needed(jobs, val_needed - jobs[&y].eval(jobs))
                    } else { 
                        jobs[&y].eval_needed(jobs, val_needed - jobs[&x].eval(jobs))
                    }
                },
                Expr::Sub(x, y)   => {
                    if jobs[&x].humn_in_chain(jobs) {
                        jobs[&x].eval_needed(jobs, val_needed + jobs[&y].eval(jobs))
                    } else { 
                        jobs[&y].eval_needed(jobs, jobs[&x].eval(jobs) - val_needed)
                    }
                },
                Expr::Mul(x, y)   => {
                    if jobs[&x].humn_in_chain(jobs) {
                        jobs[&x].eval_needed(jobs, val_needed / jobs[&y].eval(jobs))
                    } else { 
                        jobs[&y].eval_needed(jobs, val_needed / jobs[&x].eval(jobs))
                    }
                },
                Expr::Div(x, y)   => {
                    if jobs[&x].humn_in_chain(jobs) {
                        jobs[&x].eval_needed(jobs, val_needed * jobs[&y].eval(jobs))
                    } else { 
                        jobs[&y].eval_needed(jobs, jobs[&x].eval(jobs) / val_needed)
                    }
                },
                _ => panic!("Shouldn't ever get here"),
            }
        }
    }
}

pub fn get_jobs(c: &parser::Content) -> HashMap<u32, Job> {
    let mut jobs: HashMap<u32, Job> = HashMap::new();
    c.content.lines().for_each(|line| {
        let job = Job::new(line);
        jobs.insert(job.id, job);
    });
    jobs
}

pub fn execute(jobs: &HashMap<u32, Job>) -> isize {
    jobs[&str_to_u32("root")].eval(jobs)
}

pub fn execute_with_mistakes(jobs: &HashMap<u32, Job>) -> isize {
    match jobs[&str_to_u32("root")].expr {
        Expr::Add(x, y)   => {
            if jobs[&x].humn_in_chain(&jobs) {
                jobs[&x].eval_needed(&jobs, jobs[&y].eval(&jobs))
            } else { 
                jobs[&y].eval_needed(&jobs, jobs[&x].eval(&jobs))
            }
        },
        Expr::Sub(x, y)   => {
            if jobs[&x].humn_in_chain(&jobs) {
                jobs[&x].eval_needed(&jobs, jobs[&y].eval(&jobs))
            } else { 
                jobs[&y].eval_needed(&jobs, jobs[&x].eval(&jobs))
            }
        },
        Expr::Mul(x, y)   => {
            if jobs[&x].humn_in_chain(&jobs) {
                jobs[&x].eval_needed(&jobs, jobs[&y].eval(&jobs))
            } else { 
                jobs[&y].eval_needed(&jobs, jobs[&x].eval(&jobs))
            }
        },
        Expr::Div(x, y)   => {
            if jobs[&x].humn_in_chain(&jobs) {
                jobs[&x].eval_needed(&jobs, jobs[&y].eval(&jobs))
            } else { 
                jobs[&y].eval_needed(&jobs, jobs[&x].eval(&jobs))
            }
        },
        _ => panic!("Shouldn't ever get here"),
    }
}
