pub struct Snafu {
    pub val: isize,
}

impl Snafu {
    pub fn new(input: &str) -> Self {
        let val: isize = input.chars()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + Snafu::char_val(&x)*5_isize.pow(i as u32));
        Self { val }
    }

    pub fn sum(snafus: &Vec<Self>) -> Self {
        Self { val: snafus.iter().fold(0, |acc, x| acc+x.val) }
    }

    pub fn to_string(&self) -> String {
        let mut val   = self.val;
        let mut snafu = ['0'; 50];
        for c in &mut snafu {
            let x   = val + 2;
            let nbr = x.rem_euclid(5) - 2;
            val     = x.div_euclid(5);
            *c      = Snafu::val_char(&nbr);

            if val == 0 {
                break
            }
        }

        snafu.iter().rev().skip_while(|&c| *c == '0').collect()
    }

    fn char_val(c: &char) -> isize {
        match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _   => panic!("Unknown symbol")
        }
    }

    fn val_char(c: &isize) -> char {
        match c {
            -2 => '=',
            -1 => '-',
            0  => '0',
            1  => '1',
            2  => '2',
            _   => panic!("Unknown number")
        }
    }
}

