use parser;

pub struct Rucksack<'a> {
    c1: &'a str,
    c2: &'a str,
}

impl<'a> Rucksack<'a> {
    pub fn build(line: &'a str) -> Rucksack<'a> {
        Rucksack { c1: &line[..line.len()/2], c2: &line[line.len()/2..] }
    }

    pub fn find_common(&self) -> u32 {
        let res: Vec<u32> = self.c1.chars()
            .filter_map(|x|
                        if self.c2.contains(x) { // TODO Fix this later
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
