use day1;
use parser;

pub fn main() {
    let elfs = parser::Content::read_file(&"input.txt").unwrap(); // it should panic if this doesn't work
    let elf_list = day1::create_elf_list(elfs);
    let melf = day1::max_elf(&elf_list, 1);
    println!("maximum: {}", melf);
    let m3elf = day1::max_elf(&elf_list, 3);
    println!("3 maximum: {}", m3elf);
}

#[cfg(test)]
mod tests {
    use day1;

    //#[test]
    //fn elf_list_test() {
    //    let elfs = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    //    let elf_list = day1::create_elf_list(elfs.to_string());
    //    assert_eq!(elf_list[0].bag, [1000, 2000, 3000])
    //}
    //#[test]
    //fn elf_list1_test() {
    //    let elfs = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    //    let elf_list = day1::create_elf_list(elfs.to_string());
    //    assert_eq!(elf_list[1].bag, [4000])
    //}
    //#[test]
    //fn elf_list2_test() {
    //    let elfs = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    //    let elf_list = day1::create_elf_list(elfs.to_string());
    //    assert_eq!(elf_list[2].bag, [5000, 6000])
    //}
    //#[test]
    //fn elf_list3_test() {
    //    let elfs = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    //    let elf_list = day1::create_elf_list(elfs.to_string());
    //    assert_eq!(elf_list[3].bag, [7000, 8000, 9000])
    //}
    //#[test]
    //fn elf_list4_test() {
    //    let elfs = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    //    let elf_list = day1::create_elf_list(elfs.to_string());
    //    assert_eq!(elf_list[4].bag, [10000])
    //}
    #[test]
    fn elf_sum_test() {
        let elfs = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let elf_list = day1::create_elf_list(elfs.to_string());
        let melf = day1::max_elf(&elf_list, 1);
        assert_eq!(melf, 24000)
    }
    #[test]
    fn elf_sum3_test() {
        let elfs = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let elf_list = day1::create_elf_list(elfs.to_string());
        let melf = day1::max_elf(&elf_list, 3);
        assert_eq!(melf, 45000)
    }
}
