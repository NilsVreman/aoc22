use parser;

const PATH: &str = "data";
const FILENAME: &str = "day01";

pub fn main() {
    let elfs = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let elf_list = day01::create_elf_list(&elfs);
    let res_a = day01::max_elf(&elf_list, 1);
    //println!("Part A: {}", res_a);
    let res_b = day01::max_elf(&elf_list, 3);
    //println!("Part B: {}", res_b);
}

#[cfg(test)]
mod tests {
    use day01;
    use parser::Content;

    #[test]
    fn elf_sum_test() {
        let elfs = Content::read("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        let elf_list = day01::create_elf_list(&elfs);
        let melf = day01::max_elf(&elf_list, 1);
        assert_eq!(melf, 24000)
    }
    #[test]
    fn elf_sum3_test() {
        let elfs = Content::read("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        let elf_list = day01::create_elf_list(&elfs);
        let melf = day01::max_elf(&elf_list, 3);
        assert_eq!(melf, 45000)
    }
}
