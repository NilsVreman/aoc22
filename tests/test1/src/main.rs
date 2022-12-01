use std::process;
use std::env;
use test1::{nbr_floors, idx_floors, Config};

pub fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Something wrong with initial arguments: {err}");
        process::exit(1)
    });

    //println!("floor: {}", nbr_floors(config));
    println!("idx: {}", idx_floors(config));
}

#[cfg(test)]
mod tests_part1 {
    use test1::floor_count;

    #[test]
    fn seq1() {
        let result = floor_count("(())".to_string());
        assert_eq!(result, 0);
    }
    #[test]
    fn seq2() {
        let result = floor_count("()()".to_string());
        assert_eq!(result, 0);
    }
    #[test]
    fn seq3() {
        let result = floor_count("(((".to_string());
        assert_eq!(result, 3);
    }
    #[test]
    fn seq4() {
        let result = floor_count("(()(()(".to_string());
        assert_eq!(result, 3);
    }
    #[test]
    fn seq5() {
        let result = floor_count("))(((((".to_string());
        assert_eq!(result, 3);
    }
    #[test]
    fn seq6() {
        let result = floor_count("())".to_string());
        assert_eq!(result, -1);
    }
    #[test]
    fn seq7() {
        let result = floor_count("))(".to_string());
        assert_eq!(result, -1);
    }
    #[test]
    fn seq8() {
        let result = floor_count(")())())".to_string());
        assert_eq!(result, -3);
    }
    #[test]
    fn seq9() {
        let result = floor_count(")))".to_string());
        assert_eq!(result, -3);
    }
}

#[cfg(test)]
mod tests_part2 {
    use test1::floor_idx;

    #[test]
    fn seq1() {
        let result = floor_idx(")".to_string());
        assert_eq!(result, 1);
    }
    #[test]
    fn seq2() {
        let result = floor_idx("()())".to_string());
        assert_eq!(result, 5);
    }
}
