/// a parser which contains a pattern where it matches strings against delimiter "{}"
pub struct Parser<'a> {
    pattern: Vec<&'a str>,
}

impl<'a> Parser<'a> {

    pub fn build(pattern: &'a str, ph: &'a str) -> Parser {
        Parser { pattern: pattern.split(ph).collect() }
    }

    pub fn parse(&self, line: &'a str) -> Result<Vec<&'a str>, &'static str> {
        let mut i: usize = 0;
        let mut i2: usize;
        let mut res: Vec<&'a str> = Vec::new();

        for (pi, p) in self.pattern.iter().enumerate() {
            // Iterate over all but last delimiter (since it is the closing one)
            if pi == self.pattern.len()-1 { break; }

            // If one of the delimiters in the middle are empty
            if p.is_empty() && pi != 0 {
                return Err("Invalid pattern");

            // If the last element should be pattern matched, add remainder of line
            } else if pi == self.pattern.len()-2 && self.pattern[pi+1].is_empty() {
                res.push(&line[i+p.len()..]);

            } else {
                // Find index where next delimiter starts (i2)
                i2 = i + p.len() + line[i+p.len()..].find(self.pattern[pi+1])
                    .expect("No such pattern in input");
                res.push(&line[i+p.len()..i2]);
                i = i2;
            }
        }

        Ok(res)
    }
}
