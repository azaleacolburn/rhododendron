use core::ascii;
use std::num::ParseIntError;

/// Figure out a whitespace agnostic way of splitting things
/// This can be inefficient since it only has to run once
pub fn string_to_tokens(buff: String) -> Result<Vec<Token>, ParseIntError> {
    let mut token_stream: Vec<Token> = vec![];
    // let split_buff: Vec<&str> = buff.split(" ").collect::<Vec<&str>>();
    let split_buff: Vec<&str> = break_string(buff);
    for tok in split_buff.iter() {
        let mut is_dec = true;
        let chars = &tok.chars().collect::<Vec<char>>();
        chars.into_iter().for_each(|x| if !x.is_numeric() { is_dec = false; });
        if chars[0] == '0' { // handles literals
            let string = chars.into_iter().collect::<String>();
            let mut radix = 0; // 0 is not extranious base value
            match chars[1] {
                'x' => { // hex
                    radix = 12;
                },
                'o' => { // octal
                    radix = 8;
                },
                'b' => { // binary
                    radix = 2;
                },
                _ => {}
            }
            if radix != 0 {
                match i32::from_str_radix(&string, radix) {
                    Ok(value) => {
                        token_stream.push(Token::NumLiteral(value));
                    },
                    Err(err) => return Err(err),
                }
            }
            break;
        }
        if is_dec { token_stream.push(Token::NumLiteral(tok.to_string().parse::<i32>().unwrap())); }
        token_stream.push(match *tok {
            "int" => Token::Type(VariableTypes::Int),
            "char" => Token::Type(VariableTypes::Char),
            "if" => Token::If,
            "for" => Token::For,
            "while" => Token::While,
            "*" => Token::Star,
            "+" => Token::Add,
            "-" => Token::Sub,
            "/" => Token::Div,
            "+=" => Token::AddEq,
            "-=" => Token::SubEq,
            "/=" => Token::DivEq,
            "*=" => Token::MulEq,
            "==" => Token::EqCmp,
            "!=" => Token::NeqCmp,
            "|" => Token::BOr,
            "&" => Token::BAnd,
            "^" => Token::BXor,
            "|=" => Token::BOrEq,
            "&=" => Token::BAndEq,
            "^=" => Token::BXorEq,
            "(" => Token::OParen,
            ")" => Token::CParen,
            "{" => Token::OCurl,
            "}" => Token::CParen,
            ";" => Token::Semi,
            _ => Token::Id((*tok).to_string())
        });
    }
    Ok(token_stream)
}

// todo: write this function
fn break_string(buff: String) -> Vec<&'static str> {
    let split = vec![];
    let chars = buff.chars();
    let mut curr: String = String::from("");
    for c in chars {
        curr.push(c);
    }
    split
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    If,
    For,
    While,
    Type(VariableTypes),
    // Assign(String),
    Star,
    // Var(String),
    NumLiteral(i32),
    Add,
    // Mul,
    Sub,
    Div,
    Eq,
    Id(String), // why is there id and var???
    EqCmp,
    NeqCmp,
    BOr,
    BAnd,
    BXor,
    BOrEq,
    BAndEq,
    BXorEq,
    SubEq,
    AddEq,
    DivEq,
    MulEq,
    OParen,
    CParen,
    OCurl,
    CCurl,
    Semi,

    // this might be to much for the lexer to do
    // FuncDeclare((String, Vec<String>, VariableTypes)), // function name, args, return type
    // FuncCall(String, Vec<String>), // function name, args
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableTypes {
    Char,
    Int
}