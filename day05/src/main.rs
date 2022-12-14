use parser;

const PATH: &str = "data";
const FILENAME: &str = "day05";

pub fn main() {
    let c = parser::Content::read_file(PATH, FILENAME).expect("No input file found!");
    let strings = c.content.split("\n\n").collect::<Vec<&str>>();

    let mut stacks = day05::Stacks::build(&strings[0]);
    let cmds = day05::CargoMove::build_move_stack(&strings[1]);
    stacks.execute_9000(&cmds);
    //println!("Part A: {}", stacks.top_layer());

    stacks = day05::Stacks::build(&strings[0]);
    stacks.execute_9001(&cmds);
    //println!("Part B: {}", stacks.top_layer());

}

#[cfg(test)]
mod tests {
    use parser;
    use day05;

    #[test]
    fn t1_test() {
        let c = parser::Content::read(&"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2");
        let strings = c.content.split("\n\n").collect::<Vec<&str>>();
        let mut stacks = day05::Stacks::build(&strings[0]);
        let cmds = day05::CargoMove::build_move_stack(&strings[1]);
        stacks.execute_9000(&cmds);
        assert_eq!(stacks.top_layer(), "CMZ")
    }

    #[test]
    fn t2_test() {
        let c = parser::Content::read(&"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2");
        let strings = c.content.split("\n\n").collect::<Vec<&str>>();
        let mut stacks = day05::Stacks::build(&strings[0]);
        let cmds = day05::CargoMove::build_move_stack(&strings[1]);
        stacks.execute_9001(&cmds);
        assert_eq!(stacks.top_layer(), "MCD")
    }
}
