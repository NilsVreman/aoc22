use std::fs;
use std::process;

pub struct FileContent {
    content: String,
}

impl FileContent {
    pub fn build(mut args: impl Iterator<Item = String>) -> FileContent {
        args.next();

        let file_name = match args.next() {
            Some(f) => f,
            None => {
                eprintln!("Invalid file name");
                process::exit(1)
            }
        };

        let content = fs::read_to_string(file_name).unwrap_or_else(|err| {
            eprintln!("Couldn't read file: {err}");
            process::exit(1)
        });

        FileContent { content }
    }
}

pub struct Present {
   l: i32,
   w: i32,
   h: i32,
}

impl Present {
    pub fn build(line: &str) -> Present {
        let mut litr = line.split('x');
        Present { 
            l: litr.next().unwrap().parse::<i32>().unwrap(),
            w: litr.next().unwrap().parse::<i32>().unwrap(),
            h: litr.next().unwrap().parse::<i32>().unwrap(),
        }
    }

    pub fn area(&self) -> i32 {
        2*self.l*self.w + 2*self.w*self.h + 2*self.h*self.l
    }

    pub fn circumference(&self) -> i32 {
        self.l*self.w*self.h
    }

    fn min_side_area(&self) -> i32 {
        let mut min = self.l*self.w;
        min = if min <= self.w*self.h { min } else { self.w*self.h };
        min = if min <= self.h*self.l { min } else { self.l*self.h };
        min
    }

    fn min_circumference(&self) -> i32 {
        let mut min = self.l+self.w;
        min = if min <= self.w+self.h { min } else { self.w+self.h };
        min = if min <= self.h+self.l { min } else { self.l+self.h };
        2*min
    }

    pub fn area_plus_slack(&self) -> i32 {
        self.area() + self.min_side_area()
    }

    pub fn circumference_plus_slack(&self) -> i32 {
        self.circumference() + self.min_circumference()
    }
}

pub fn get_present_list(fc: FileContent) -> Vec<Present> {
    fc.content.lines().map(|x| Present::build(x)).collect()
}
