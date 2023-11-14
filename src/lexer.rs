use core::ascii;
use std::num::ParseIntError;

/// Figure out a whitespace agnostic way of splitting things
/// This can be inefficient since it only has to run once
pub fn string_to_tokens(buff: String) -> Result<Vec<Token>, ParseIntError> {
    let mut token_stream: Vec<Token> = vec![];
    // let split_buff: Vec<&str> = buff.split(" ").collect::<Vec<&str>>();
    // let split_buff: Vec<Token> = break_string(buff);
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
        token_stream.push(match tok.as_str() {
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

/// This is where the lexical analysis happens
pub fn break_string(buff: String) -> Vec<Token> {
    let mut split: Vec<String> = vec![];
    let ret: Vec<Token> = vec![];
    let chars = buff.chars().collect::<Vec<char>>();
    let mut curr: String = String::from("");
    for mut i in 0..chars.len() {
        match chars[i] {
            'i' => {
                if chars[i + 1] == 'n' && chars[i + 2] == 't' && chars[i + 3] == ' ' {
                    // split.push(String::from("int"));
                    ret.push(Token::Type(VariableTypes::Int));
                    i += 3;
                } else if chars[i + 1] == 'f' && chars[i + 2] == ' ' {
                    // split.push(String::from("if"));
                    ret.push(Token::If);
                    i += 2; // these numbers might be wrong
                }
            },
            'c' => {
                if chars[i + 1] == 'h' && chars[i + 2] == 'a' && chars[i + 3] == 'r' && chars[i + 4] == ' ' {
                    // split.push(String::from("char"));
                    ret.push(Token::Type(VariableTypes::Char));
                    i += 4;
                }
            },
            'f' => {
                if chars[i + 1] == 'o' && chars[i + 2] == 'r' && chars[i + 3] == ' ' {
                    // split.push(String::from("for"));
                    ret.push(Token::For);
                    i += 3;
                }
            },
            'l' => {
                if chars[i + 1] == 'o' && chars[i + 2] == 'o' && chars[i + 3] == 'p' && chars[i + 4] == ' ' {
                    // split.push(String::from("loop"));
                    ret.push(Token::Loop);
                    i += 4;
                }
            },
            '+' => {
                if chars[i + 1] == '=' { 
                    //split.push(String::from("+=")); 
                    ret.push(Token::AddEq)
                }
                else if chars[i + 1] == '+' { 
                    //split.push(String::from("++")); 
                    ret.push(Token::AddO);
                }
                else { 
                    //split.push(String::from("+")); 
                    ret.push(Token::Add);
                }
            },
            '-' => {
                if chars[i + 1] == '=' { 
                    //split.push(String::from("-="));
                    ret.push(Token::SubEq);
                }
                else if chars[i + 1] == '-' {
                    // split.push(String::from("--"));
                    ret.push(Token::SubO);
                }
                else {
                    // split.push(String::from("-"));
                    ret.push(Token::Sub);
                }
            },
            '/' => {
                if chars[i + 1] == '=' {
                    //split.push(String::from("/="));
                    ret.push(Token::SubEq)
                }
                else {
                    // split.push(String::from("/"));
                    ret.push(Token::Div);
                }
            },
            '*' => {
                if chars[i + 1] == '*' {
                    // split.push(String::from("*="));
                    ret.push(Token::MulEq);
                } // this could probably also handle deref vs. mul
                else {
                    // split.push(String::from("*"));
                    ret.push(Token::Star); // The lexer can probably determine whether this is a mul or deref
                }
            },
            // obviously none of this can be included in ids
            '(' => {
                // split.push(String::from("("));
                ret.push(Token::OParen);
            },
            ')' => {
                // split.push(String::from(")"));
                ret.push(Token::CParen);
            },
            '{' => {
                // split.push(String::from("{"));
                ret.push(Token::OCurl);
            },
            '}' => {
                //split.push(String::from("}"));
                ret.push(Token::CCurl);
            },
            '&' => {
                // split.push(String::from("&"));
                if chars[i + 1] == '=' {
                    ret.push(Token::BAndEq);
                } else {
                    ret.push(Token::BAnd);
                }
            },
            '^' => {
                if chars[i + 1] == '=' {
                    ret.push(Token::BXorEq);
                } else {
                    ret.push(Token::BXor);
                }
                // split.push(String::from("^"));
            },
            '%' => {
                // split.push(String::from("%"));
                if chars[i + 1] == '=' {
                    ret.push(Token::ModEq);
                } else {
                    ret.push(Token::Mod);
                }
            },
            '!' => {
                // split.push(String::from("!"));
                if chars[i + 1] == '=' {
                    ret.push(Token::NeqCmp);
                } else {
                    ret.push(Token::Neq);
                }
            },
            '|' => {
                // split.push(String::from("|"));
                if chars[i + 1] == '=' {
                    ret.push(Token::BOrEq);
                } else {
                    ret.push(Token::BOr);
                }
            },
            '~' => {
                // split.push(String::from("~"));
                if chars[i + 1] == '=' {
                    ret.push(Token::BNotEq);
                } else {
                    ret.push(Token::BNot);
                }
            },
            '<' => {
                // split.push(String::from("<"));
                if chars[i + 1] == '=' {
                    ret.push(Token::LsEq);
                } else {
                    ret.push(Token::Ls);
                }
            },
            '>' => {
                // split.push(String::from(">"));
                if chars[i + 1] == '=' {
                    ret.push(Token::GrEq);
                } else {
                    ret.push(Token::Gr);
                }
            },
            '.' => {
                // split.push(String::from("."));
                ret.push(Token::Dot);
            },
            ',' => {
                // split.push(String::from(","));
                ret.push(Token::Comma);
            },
            ';' => {
                // split.push(String::from(";"));
                ret.push(Token::Semi);
            },
            _ => {
                // if we'e here it's an identifier
                for i in i..chars.len() {
                    curr.push(chars[i]);
                    if chars[i] == ' ' { break; }
                }
                split.push(curr.clone());
            }
        }
        // curr.push(c);
    }
    split
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    If,
    For,
    While,
    Loop,
    Type(VariableTypes),
    // Assign(String),
    Star,
    // Var(String),
    NumLiteral(i32),
    Add,
    AddO,
    // Mul,
    Sub,
    SubO,
    Div,
    Mod,
    ModEq,
    Eq,
    Id(String), // why is there id and var???
    EqCmp,
    NeqCmp,
    Neq,
    BOr,
    BAnd,
    BXor,
    BOrEq,
    BAndEq,
    BXorEq,
    BNot,
    BNotEq,
    SubEq,
    AddEq,
    DivEq,
    MulEq,
    LsEq,
    Ls,
    Gr,
    GrEq,
    OParen,
    CParen,
    OCurl,
    CCurl,
    Dot,
    Comma
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