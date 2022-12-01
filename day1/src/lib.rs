use parser;

pub struct Elf {
    bag: Vec<i32>,
}

impl Elf {
    pub fn sum(&self) -> i32 {
        self.bag.iter().sum()
    }
}

pub fn create_elf_list(c: parser::Content) -> Vec<Elf> {
    let mut res: Vec<Elf> = Vec::new();
    let lines: Vec<&str> = c.content.lines().collect();
    let mut i: usize = 0;

    loop {
        let mut bag: Vec<i32> = Vec::new();

        while i < lines.len() && !lines[i].is_empty() {
            bag.push(lines[i].parse::<i32>().unwrap());
            i += 1;
        }

        res.push( Elf { bag } );

        i += 1;

        if i >= lines.len() { break; }
    }

    res
}

pub fn max_elf(elfs: &Vec<Elf>, n: usize) -> i32 {
    let mut calories: Vec<i32> = elfs.iter().map(|x| x.sum()).collect();
    calories.sort_by(|a,b| b.cmp(a));
    calories[0..n].iter().sum()
}

