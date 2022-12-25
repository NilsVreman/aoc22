use parser;

const PATH: &str = "../data";
const FILENAME: &str = "day25";


pub fn main() {
    let cont  = parser::Content::read_file(PATH, FILENAME).expect("No input file found");
    let res_a = day25::Snafu::sum(&cont.content.lines()
                                  .map(|x| day25::Snafu::new(x))
                                  .collect::<Vec<_>>());
    println!("Part A: {}", res_a.to_string());
}

#[cfg(test)]
mod tests {
    use parser;
    use day25;

    #[test]
    fn t1_test() {
        let cont = parser::Content::read(&"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122");
        let res = day25::Snafu::sum(&cont.content.lines().map(|x| day25::Snafu::new(x)).collect::<Vec<_>>());
        assert_eq!(res.to_string(), "2=-1=0".to_string())
    }
}
