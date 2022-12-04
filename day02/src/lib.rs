enum Shape {
    R = 1,
    P = 2,
    S = 3,
}

enum Outcome {
    L = 0,
    W = 6,
    D = 3,
}

impl Shape {
    pub fn build(s: &str) -> Shape {
        match s {
            "A" | "X" => Shape::R,
            "B" | "Y" => Shape::P,
            "C" | "Z" => Shape::S,
            _ => panic!("No such str representing a shape"),
        }
    }

    pub fn build_from_answer(s: &str, other: &Shape) -> Shape {
        match (other, s) {
            (Shape::R, "Y") | (Shape::P, "X") | (Shape::S, "Z") => Shape::R,
            (Shape::R, "Z") | (Shape::P, "Y") | (Shape::S, "X") => Shape::P,
            (Shape::R, "X") | (Shape::P, "Z") | (Shape::S, "Y") => Shape::S,
            _ => panic!("No such str representing a shape"),
        }
    }

    pub fn game(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (Shape::R, Shape::R) | (Shape::P, Shape::P) | (Shape::S, Shape::S) => Outcome::D,
            (Shape::R, Shape::S) | (Shape::P, Shape::R) | (Shape::S, Shape::P) => Outcome::W,
            (Shape::R, Shape::P) | (Shape::P, Shape::S) | (Shape::S, Shape::R) => Outcome::L,
        }
    }

    pub fn points(&self, other: &Shape) -> u32 {
        self.game(&other) as u32 + *self as u32
    }
}

pub fn game(arg_list: &Vec<&str>) -> u32 {
    let other_choice = Shape::build(arg_list[0]);
    let my_choice = Shape::build(arg_list[1]);

    my_choice.points(&other_choice)
}

pub fn game_from_answer(arg_list: &Vec<&str>) -> u32 {
    let other_choice = Shape::build(arg_list[0]);
    let my_choice = Shape::build_from_answer(arg_list[1], &other_choice);

    my_choice.points(&other_choice)
}
