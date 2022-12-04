use parser;

pub struct Pair {
    e1: (u32, u32),
    e2: (u32, u32),
}

impl Pair {
    pub fn build(line: &str) -> Pair {
        let ps = parser::Parser::build("{}-{},{}-{}", "{}");
        let res: Vec<u32> = ps.parse(&line)
            .unwrap() //should panic if can't unwrap
            .iter()
            .map(|x| x.parse::<u32>().expect("Couldn't parse input"))
            .collect();
        Pair { e1: (res[0], res[1]), e2: (res[2], res[3]) }
    }

    pub fn contains_other(&self) -> bool {
        (self.e1.0..=self.e1.1).contains(&self.e2.0) 
            || (self.e1.0..=self.e1.1).contains(&self.e2.1)
            || (self.e2.0..=self.e2.1).contains(&self.e1.0)
            || (self.e2.0..=self.e2.1).contains(&self.e1.1)
    }

    pub fn fully_contains_other(&self) -> bool {
        ((self.e1.0..=self.e1.1).contains(&self.e2.0)
         && (self.e1.0..=self.e1.1).contains(&self.e2.1))
            || ((self.e2.0..=self.e2.1).contains(&self.e1.0)
                && (self.e2.0..=self.e2.1).contains(&self.e1.1))
    }
}

pub fn pairs_vec(c: &parser::Content) -> Vec<Pair> {
    c.content.lines().map(|x| Pair::build(&x)).collect()
}
