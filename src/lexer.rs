pub use self::Token::{
    Sp,
    Ed,
    OpPrn,
    ClPrn,
    Lt,
    Id,
    Pl
};

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Sp,
    Ed,
    OpPrn,
    ClPrn,
    pub enum Lt {
        N(f64),
        S(String),
    },
    Id(String),
    Pl(String)
}


pub fn tokenize(input: &str) -> Vec<Token> {

    let comment_re = regex!(r"(?m)#.*\n");
    let preprocessed = comment_re.replace_all(input, "\n");
    let mut result = Vec::new();

    let tokens = regex!(concat!(
        r"(?P<sp> +)|",
        r"(?P<ed>\n+)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<id>[A-z]+\w)|",
        r"(?P<n>\d+\.?\d*)|",
        r"(?P<s>'.*'+)|",
        r"(?P<Pl>\S)"
    ));

    for cap in tokens.captures_iter(preprocessed.as_str()) {
        let token = if cap.name("id").is_some() {
            match cap.name("id").unwrap() {
                id => Id(ident.to_string())
            }
        } else if cap.name("n").is_some() {
            match cap.name("n").unwrap().parse() {
                Ok(n) => Lt::N(n),
                Err(n) => panic!("Lexer failed trying to parse number", n)
            }
        } else if cap.name("s").is_some() {
            Lt::S(cap.name("s").unwrap().to_string())
        } else if cap.name("sp").is_some() {
            Sp
        } else if cap.name("ed").is_some() {
            Ed
        } else if cap.name("oppar").is_some() {
            OpPrn
        } else if cap.name("clpar").is_some() {
            ClPrn
        } else {
            Pl(cap.name("operator").unwrap().to_string())
        };

        result.push(token)
    }

    result
}
