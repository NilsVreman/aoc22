use parser::Parser;

pub fn main() {
    let pattern = "{}x: {} {}, px{}";
    let placeholder = "{}";
    let ps = Parser::build(&pattern, &placeholder);

    let input = "1.83409x: 82093 kdjflk, pxhellothere";
    let res = ps.parse(&input);
    println!("{:?}", res);
}

#[cfg(test)]
mod tests {
    use parser::Parser;

    //#[test]
    //fn randomstring1_test() {
    //    let pattern = "{}x: {} {}, px";
    //    let placeholder = "{}";
    //    let ps = Parser::build(&pattern, &placeholder);
    //    assert_eq!(ps.pattern, ["", "x: ", " ", ", px"])
    //}

    //#[test]
    //fn randomstring2_test() {
    //    let pattern = "{}";
    //    let placeholder = "{}";
    //    let ps = Parser::build(&pattern, &placeholder);
    //    assert_eq!(ps.pattern, ["", ""])
    //}

    //#[test]
    //fn randomstring3_test() {
    //    let pattern = "{} x {} y ";
    //    let placeholder = "{}";
    //    let ps = Parser::build(&pattern, &placeholder);
    //    assert_eq!(ps.pattern, ["", " x ", " y "])
    //}

    //#[test]
    //fn randomstring4_test() {
    //    let pattern = "{}x: {} {}, px{}";
    //    let placeholder = "{}";
    //    let ps = Parser::build(&pattern, &placeholder);
    //    assert_eq!(ps.pattern, ["", "x: ", " ", ", px", ""])
    //}

    //#[test]
    //fn randomstring5_test() {
    //    let pattern = "{}x{}x{}";
    //    let placeholder = "{}";
    //    let ps = Parser::build(&pattern, &placeholder);
    //    assert_eq!(ps.pattern, ["", "x", "x", ""])
    //}
    
    #[test]
    fn parse1_test() {
        let pattern = "{}x: {} {}, px{}";
        let placeholder = "{}";
        let ps = Parser::build(&pattern, &placeholder);

        let input = "1.83409x: 82093 kdjflk, pxhellothere";
        assert_eq!(ps.parse(&input).unwrap(), &["1.83409", "82093", "kdjflk", "hellothere"])
    }

    #[test]
    fn parse2_test() {
        let pattern = "{}";
        let placeholder = "{}";
        let ps = Parser::build(&pattern, &placeholder);

        let input = "1.83409";
        assert_eq!(ps.parse(&input).unwrap(), &["1.83409"])
    }

    #[test]
    fn parse3_test() {
        let pattern = "{}x{}x{}";
        let placeholder = "{}";
        let ps = Parser::build(&pattern, &placeholder);

        let input = "1x1x10";
        assert_eq!(ps.parse(&input).unwrap(), &["1", "1", "10"])
    }

    #[test]
    fn parse4_test() {
        let pattern = "{}x{}x{}";
        let placeholder = "{}";
        let ps = Parser::build(&pattern, &placeholder);

        let input = "2x3x4";
        assert_eq!(ps.parse(&input).unwrap(), &["2", "3", "4"])
    }
}
