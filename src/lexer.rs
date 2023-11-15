use std::num::ParseIntError;

/// This is where the lexical analysis happens
pub fn string_to_tokens(buff: &String) -> Result<Vec<Token>, ParseIntError> {
    let mut ret: Vec<Token> = vec![];
    let chars = buff.chars().collect::<Vec<char>>();
    let mut curr: String = String::from("");
    let mut i: usize = 0;
    while i < chars.len() {
        // Handles num literals but we don't actually know if it is a literal yet
        if chars[i].is_numeric() {
            let mut is_dec = true;
            // chars.into_iter().for_each(|x| if !x.is_numeric() { is_dec = false; });
            let mut num = String::from("");
            for j in i..chars.len() {
                if !chars[j].is_alphanumeric() { break; }
                if chars[j].is_alphabetic() && chars[j].is_uppercase() { is_dec = false; }
                num.push(chars[j]);
            }
            println!("num: {}", num);
            println!("is dec: {}", is_dec);
            if chars[i] == '0' { // handles literals // TODO: DO LITERAL SHIT
                // let string = chars.into_iter().collect::<String>();
                
                let mut radix = 0; // 0 is not extranious base value
                match chars[i + 1] {
                    'x' => { // hex
                        radix = 12;
                    },
                    'o' => { // octal
                        radix = 8;
                    },
                    'b' => { // binary
                        radix = 2;
                    },
                    _ => {
                        if chars[i + 1].is_alphabetic() {
                            panic!("Not supported base")
                        }
                    }
                }
                if radix != 0 {
                    match i32::from_str_radix(&num, radix) {
                        Ok(value) => {
                            ret.push(Token::NumLiteral(value));
                        },
                        Err(err) => { continue; },
                    };
                    i += 1;
                    continue;
                }
            }
            if is_dec { ret.push(Token::NumLiteral(num.parse::<i32>().unwrap())); i += 1; continue; }

        }
        
        println!("char: {}", chars[i]);
        match chars[i] {
            ' ' => {},
            'i' => {
                if chars[i + 1] == 'n' && chars[i + 2] == 't' && chars[i + 3] == ' ' {
                    println!("here in int");
                    // split.push(String::from("int"));
                    ret.push(Token::Type(VariableTypes::Int));
                    i += 2; // I think there's a problem with incrementing the iterator
                } else if chars[i + 1] == 'f' && chars[i + 2] == ' ' {
                    // split.push(String::from("if"));
                    ret.push(Token::If);
                    i += 1; // these numbers might be wrong
                }
            },
            'c' => {
                if chars[i + 1] == 'h' && chars[i + 2] == 'a' && chars[i + 3] == 'r' && chars[i + 4] == ' ' {
                    // split.push(String::from("char"));
                    ret.push(Token::Type(VariableTypes::Char));
                    i += 2;
                }
            },
            'f' => {
                if chars[i + 1] == 'o' && chars[i + 2] == 'r' && chars[i + 3] == ' ' {
                    // split.push(String::from("for"));
                    ret.push(Token::For);
                    i += 2;
                }
            },
            'l' => {
                if chars[i + 1] == 'o' && chars[i + 2] == 'o' && chars[i + 3] == 'p' && chars[i + 4] == ' ' {
                    // split.push(String::from("loop"));
                    ret.push(Token::Loop);
                    i += 3;
                }
            },
            '+' => {
                if chars[i + 1] == '=' { 
                    //split.push(String::from("+=")); 
                    ret.push(Token::AddEq);
                    i += 1;
                }
                else if chars[i + 1] == '+' { 
                    //split.push(String::from("++")); 
                    ret.push(Token::AddO);
                    i += 1;
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
                    i += 1;
                }
                else if chars[i + 1] == '-' {
                    // split.push(String::from("--"));
                    ret.push(Token::SubO);
                    i += 1;
                }
                else {
                    // split.push(String::from("-"));
                    ret.push(Token::Sub);
                }
            },
            '/' => {
                if chars[i + 1] == '=' {
                    //split.push(String::from("/="));
                    ret.push(Token::SubEq);
                    i += 1;
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
                    i += 1;
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
                    i += 1;
                } else {
                    ret.push(Token::BAnd);
                }
            },
            '^' => {
                if chars[i + 1] == '=' {
                    ret.push(Token::BXorEq);
                    i += 1;
                } else {
                    ret.push(Token::BXor);
                }
                // split.push(String::from("^"));
            },
            '%' => {
                // split.push(String::from("%"));
                if chars[i + 1] == '=' {
                    ret.push(Token::ModEq);
                    i += 1;
                } else {
                    ret.push(Token::Mod);
                }
            },
            '!' => {
                // split.push(String::from("!"));
                if chars[i + 1] == '=' {
                    ret.push(Token::NeqCmp);
                    i += 1;
                } else {
                    ret.push(Token::Neq);
                }
            },
            '|' => {
                // split.push(String::from("|"));
                if chars[i + 1] == '=' {
                    ret.push(Token::BOrEq);
                    i += 1;
                } else {
                    ret.push(Token::BOr);
                }
            },
            '~' => {
                // split.push(String::from("~"));
                if chars[i + 1] == '=' {
                    ret.push(Token::BNotEq);
                    i += 1;
                } else {
                    ret.push(Token::BNot);
                }
            },
            '<' => {
                // split.push(String::from("<"));
                if chars[i + 1] == '=' {
                    ret.push(Token::LsEq);
                    i += 1;
                } else {
                    ret.push(Token::Ls);
                }
            },
            '>' => {
                // split.push(String::from(">"));
                if chars[i + 1] == '=' {
                    ret.push(Token::GrEq);
                    i += 1;
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
            '=' => {
                if chars[i + 1] == '=' {
                    ret.push(Token::EqCmp);
                } else {
                    ret.push(Token::Eq);
                }
            }
            _ => {
                // if we'e here it's an identifier
                for j in i..chars.len() {
                    if !chars[j].is_alphabetic() && chars[j] != '_' { break; }
                    curr.push(chars[j]);
                }
                ret.push(Token::Id(curr.clone()));
                println!("curr before overflow: {}", curr);
                i += curr.len() - 1;
                curr = String::from("");
            }
        }
        i += 1;
        // curr.push(c);
    }
    Ok(ret)
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
    Comma,
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