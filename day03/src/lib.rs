use parser;

pub struct Rucksack<'a> {
    c: &'a str,
}

impl<'a> Rucksack<'a> {
    pub fn build(line: &'a str) -> Rucksack<'a> {
        Rucksack { c: &line }
    }

    pub fn find_common(&self) -> u32 {
        let left = &self.c[..self.c.len()/2];
        let right = &self.c[self.c.len()/2..];
        let res: Vec<u32> = left.chars()
            .filter_map(|x|
                        if right.contains(x) { // TODO Fix this later
                            if x.is_uppercase() {
                                Some(x as u32 + 27 - 'A' as u32)
                            } else {
                                Some(x as u32 + 1 - 'a' as u32)
                            }
                        } else {
                            None
                        }).collect();
        res[0]
    }
}

pub fn rucksacks(c: &parser::Content) -> Vec<Rucksack> {
    c.content.lines().map(|x| Rucksack::build(&x)).collect()
}

pub fn find_common_group(v: &[Rucksack]) -> u32 {
    let res: Vec<u32> = v[0].c.chars().filter_map(|x|
                    if v[1].c.contains(x) && v[2].c.contains(x) { // TODO Fix this later
                        if x.is_uppercase() {
                            Some(x as u32 + 27 - 'A' as u32)
                        } else {
                            Some(x as u32 + 1 - 'a' as u32)
                        }
                    } else {
                        None
                    }).collect();
    res[0]
}
