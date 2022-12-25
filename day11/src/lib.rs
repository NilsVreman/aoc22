use parser;

enum Expr {
    Add(usize),
    Mul(usize),
    Pow,
}

impl Expr {
    fn eval(&self, x: usize) -> usize {
        match self {
            Expr::Add(y) => x + y,
            Expr::Mul(y) => x * y,
            Expr::Pow    => x * x,
        }
    }
}

pub struct Monkey {
    id: usize,
    items: Vec<usize>,
    inspected: usize,
    operation: Expr,
    pub divisor: usize,
    targets: (usize, usize), // (true id, false id)
}

impl Monkey {
    pub fn new(input: &str) -> Self {
        let ps = parser::Parser::build("Monkey {}:
  Starting items: {}
  Operation: new = {}
  Test: divisible by {}
    If true: throw to monkey {}
    If false: throw to monkey {}", "{}");

        let parsed_vec = ps.parse(input).expect("Couldn't parse Monkey");
        let id  = parsed_vec[0].parse::<usize>().unwrap();
        let item_list  = parsed_vec[1].split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let operation = Monkey::get_op(&parsed_vec[2]);
        let divisor = parsed_vec[3].parse::<usize>().unwrap();
        let targets = (parsed_vec[4].parse::<usize>().unwrap(), parsed_vec[5].parse::<usize>().unwrap());
        Self {
            id,
            items: item_list,
            inspected: 0,
            operation,
            divisor,
            targets,
        }
    }

    fn get_op(op: &str) -> Expr {
        let op = op.split(' ').collect::<Vec<_>>();
        if op.len() != 3 { panic!("Should be 3 arguments"); }
        match (op[0], op[1], op[2]) {
             ("old", "+", "old") => Expr::Mul(2),
             ("old", "*", "old") => Expr::Pow,
             ("old", "+", y) | (y, "+", "old") => { let y = y.parse::<usize>().unwrap(); Expr::Add(y) },
             ("old", "*", y) | (y, "*", "old") => { let y = y.parse::<usize>().unwrap(); Expr::Mul(y) },
             _ => panic!("Unknown input to operation function")
        }
    }

    pub fn monkey_list(cont: &parser::Content) -> Vec<Monkey> {
        cont.content
            .split("\n\n")
            .map(|x| Monkey::new(x.trim_end()))
            .collect::<Vec<_>>()
    }

    pub fn print(&self) {
        println!("Monkey {}:", self.id);
        println!("  Items: {:?}", self.items);
        println!("  Operation: {}", match self.operation {
            Expr::Add(x) => format!("old + {x}"),
            Expr::Mul(x) => format!("old * {x}"),
            Expr::Pow    => format!("old * old"),
        });
        println!("  Test: divisible by {}", self.divisor);
        println!("    If true: throw to Monkey {}", self.targets.0);
        println!("    If false: throw to Monkey {}", self.targets.1);
    }
}

pub fn rounds<T>(mut monkeys: Vec<Monkey>,
                 n_rounds: usize,
                 worry_reduction: T)
-> usize 
where
    T: Fn(usize) -> usize,
{
    for _ in 0..n_rounds {
        for id in 0..monkeys.len() {
            let mut monkey = &mut monkeys[id];
            let items = monkey.items
                .drain(..)
                .map(|x| worry_reduction(monkey.operation.eval(x)))
                .collect::<Vec<_>>();

            monkey.inspected += items.len();

            let target_true = monkey.targets.0;
            let target_false = monkey.targets.1;
            let divisor = monkey.divisor;
            for worry in items {
                monkeys[if worry % divisor == 0 { target_true } else { target_false }].items.push(worry);
            }
        }
    }
    product_max_n(&monkeys, 2)
}

fn product_max_n(monkeys: &Vec<Monkey>, n: usize) -> usize {
    let mut res: Vec<usize> = vec![usize::MIN; n];
    for monkey in monkeys {
        let m = res.iter().min().unwrap();
        if &monkey.inspected > m {
            let pos = res.iter().position(|x| x == m).unwrap();
            res[pos] = monkey.inspected;
        }
    }
    res.iter().fold(1, |acc, x| acc*x)
}
