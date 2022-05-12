pub use self::Token::{
    Sp,
    End,
    OpPrn,
    ClPrn,
    Comma,
    Collon,
    At,
    Ident,
    Number,
    String,
    Operator
};

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Sp,
    End,
    OpPrn,
    ClPrn,
    Comma,
    Collon,
    At,
    String(String),
    Ident(String),
    Number(f64),
    Operator(String)
}

pub fn tokenize(input: &str) -> Vec<Token> {

    let comment_re = regex!(r"(?m)#.*\n");
    let preprocessed = comment_re.replace_all(input, "\n");
    let mut result = Vec::new();

    let token_re = regex!(concat!(
        r"(?P<sp> +)|",
        r"(?P<lf>\n+)|",
        r"(?P<ident>[A-z]+\w)|",
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<string>'.*'+)|"
        r"(?P<at>@)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<comma>,)|",
        r"(?P<operator>\S)"
    ));

    for cap in token_re.captures_iter(preprocessed.as_str()) {
        let token = if cap.name("ident").is_some() {
            match cap.name("ident").unwrap() {
                ident => Ident(ident.to_string())
            }
        } else if cap.name("number").is_some() {
            match cap.name("number").unwrap().parse() {
                Ok(number) => Number(number),
                Err(_) => panic!("Lexer failed trying to parse number")
            }
        } else if cap.name("string").is_some() {
            String(cap.name("string").unwrap().to_string())
        } else if cap.name("sp").is_some() {
            Sp
        } else if cap.name("lf").is_some() {
            Lf
        } else if cap.name("at").is_some() {
            At
        } else if cap.name("oppar").is_some() {
            OpPrn
        } else if cap.name("clpar").is_some() {
            ClPrn
        } else if cap.name("comma").is_some() {
            Comma
        } else {
            Operator(cap.name("operator").unwrap().to_string())
        };

        result.push(token)
    }

    result
}
