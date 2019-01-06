
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Def,
    Extern,
    Delimiter, // ';'
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String)
}

pub use self::Token::{
    Def,
    Extern,
    Delimiter,
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident,
    Number,
    Operator
};

pub fn tokenize(input: &str) -> Vec<Token> {
    let comment_re = Regex::new(r"(?m)#.*\n").unwrap();
    let input = comment_re.replace_all(input, "\n");

    let mut tokens = vec![];

    let token_re = Regex::new(concat!(
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<delimiter>;)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<comma>,)|",
        r"(?P<operator>\S)",
    )).unwrap();

    for cap in token_re.captures_iter(&input) {
        let token = if let Some(m) = cap.name("ident") {
            match m.as_str() {
                "def" => Def,
                "extern" => Extern,
                ident => Ident(ident.to_string())
            }
        } else if let Some(m) = cap.name("number") {
            Number(m.as_str().parse().expect("Lexed failed to parse number"))
        } else if let Some(_) = cap.name("delimiter") {
            Delimiter
        } else if let Some(_) = cap.name("oppar") {
            OpeningParenthesis
        } else if let Some(_) = cap.name("clpar") {
            ClosingParenthesis
        } else if let Some(_) = cap.name("comma") {
            Comma
        } else if let Some(m) = cap.name("operator"){
            Operator(m.as_str().to_string())
        } else {
            unreachable!()
        };

        tokens.push(token);
    }

    tokens
}
