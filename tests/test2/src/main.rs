use std::env;
use test2::{get_present_list, FileContent, Present};

pub fn main() {
    let fc = FileContent::build(env::args());

    let presents = get_present_list(fc);

    let area: i32 = presents.iter().map(|x| x.area_plus_slack()).sum();
    let circ: i32 = presents.iter().map(|x| x.circumference_plus_slack()).sum();

    println!("Total area: {}", area);
    println!("Total circumference: {}", circ);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area1_test() {
        let p = Present::build("2x3x4");
        assert_eq!(p.area_plus_slack(), 58)
    }
    #[test]
    fn area2_test() {
        let p = Present::build("1x1x10");
        assert_eq!(p.area_plus_slack(), 43)
    }
    #[test]
    fn circ1_test() {
        let p = Present::build("2x3x4");
        assert_eq!(p.circumference_plus_slack(), 34)
    }
    #[test]
    fn circ2_test() {
        let p = Present::build("1x1x10");
        assert_eq!(p.circumference_plus_slack(), 14)
    }
}
