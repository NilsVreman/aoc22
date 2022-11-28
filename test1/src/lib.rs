use std::process;
use std::fs;


pub fn floor_count(content: String) -> i32 {
    content.chars()
        .map(|x| if x == '(' { 1 } else { -1 })
        .sum()
}

pub fn floor_idx(content: String) -> usize {
    let v: Vec<i32> = content.chars()
        .map(|x| if x == '(' { 1 } else { -1 })
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    v.iter().position(|&x| x == -1).unwrap_or_else(|| {
        eprintln!("Error");
        process::exit(1)
    }) + 1
}

pub fn nbr_floors(config: Config) -> i32 {
    let content = fs::read_to_string(config.file_name).unwrap_or_else(|err| {
        eprintln!("floor_count: {err}");
        process::exit(1)
    });

    floor_count(content)
}

pub fn idx_floors(config: Config) -> usize {
    let content = fs::read_to_string(config.file_name).unwrap_or_else(|err| {
        eprintln!("floor_count: {err}");
        process::exit(1)
    });

    floor_idx(content)
}

pub struct Config {
    file_name: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip first element in argument iterator

        let file_name = match args.next() {
            None => return Err("No input file provided!"),
            Some(s) => s,
        };

        Ok(Config { file_name })
    }
}
