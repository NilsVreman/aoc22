use parser;

const PATH: &str = "data";
const FILENAME: &str = "day07";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let root = day07::build_tree(&c);
    let mut res: Vec<u32> = Vec::new();
    day07::size_list(&root, &mut res);
    //println!("Part A: {:?}", day07::sum_below_x(&res, 100_000));

    //println!("Part B: {}", day07::size_of_dir_to_remove(&res, 70_000_000, 30_000_000));

}

#[cfg(test)]
mod tests {
    use parser;
    use day07;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k");
        let root = day07::build_tree(&c);
        let mut res: Vec<u32> = Vec::new();
        day07::size_list(&root, &mut res);
        assert_eq!(day07::sum_below_x(&res, 100_000), 95437)
    }

    #[test]
    fn t2_test() {
        let c = parser::Content::read(&"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k");
        let root = day07::build_tree(&c);
        let mut res: Vec<u32> = Vec::new();
        day07::size_list(&root, &mut res);
        assert_eq!(*day07::size_of_dir_to_remove(&res, 70_000_000, 30_000_000), 24933642)
    }
}
