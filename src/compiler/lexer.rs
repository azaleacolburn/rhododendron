use std::{collections::HashMap, num::ParseIntError};

/// key: token_num, value: line_num
pub struct LineNumHandler {
    tokens_to_lines: HashMap<usize, usize>,
    curr_line: usize,
}

impl LineNumHandler {
    fn new() -> LineNumHandler {
        LineNumHandler {
            tokens_to_lines: HashMap::new(),
            curr_line: 0,
        }
    }

    /// Creates a new line with the start of the line being this token_number
    fn set_token_line(&mut self, token_number: usize) {
        self.tokens_to_lines.insert(token_number, self.curr_line);
    }

    fn new_line(&mut self) {
        self.curr_line += 1;
    }

    // Gets a token's line given it's index
    pub fn get_token_line(&self, token_number: usize) -> usize {
        *self
            .tokens_to_lines
            .get(&token_number)
            .expect("Invalid token index")
    }
}

/// This is where the lexical analysis happens
pub fn string_to_tokens(
    buff: impl ToString,
) -> Result<(Vec<Token>, LineNumHandler), ParseIntError> {
    let mut ret: Vec<Token> = vec![];
    let chars = buff.to_string().chars().collect::<Vec<char>>();
    let mut i: usize = 0;
    let mut line_tracker = LineNumHandler::new();
    line_tracker.new_line();
    while i < chars.len() {
        if chars[i].is_numeric() {
            let mut num = String::new();
            let radix = if chars[i] == '0' {
                i += 1;
                match chars[i] {
                    'x' => 12,
                    'o' => 8,
                    'b' => 2,
                    'a' | 'c'..='n' | 'p'..='w' | 'y' | 'z' => {
                        panic!("Unsupported base");
                    }
                    _ => {
                        num.push_str("0");
                        10
                    }
                }
            } else {
                10
            };

            while chars[i].is_alphanumeric() {
                num.push(chars[i]);
                i += 1;
            }

            let value = i32::from_str_radix(&num, radix)
                .expect("Invalid numeric literal. Probably a lexing error");

            line_tracker.set_token_line(ret.len());
            ret.push(Token::NumLiteral(value));
            continue;
        }

        match chars[i] {
            ' ' => {}
            '\"' => {
                let mut str = String::new();
                i += 1;
                while chars[i] != '\"' {
                    str.push(chars[i]);
                    i += 1;
                }
                ret.push(Token::StrLiteral(str));
                line_tracker.set_token_line(ret.len());
            }
            'A'..='Z' | 'a'..='z' | '-' | '_' => {
                let token: Option<Token> = match chars[i] {
                    'i' => {
                        if chars[i + 1] == 'n' && chars[i + 2] == 't' && chars[i + 3] == ' ' {
                            line_tracker.set_token_line(ret.len());
                            i += 2;
                            Some(Token::Type(RhTypes::Int))
                        } else if chars[i + 1] == 'f'
                            && (chars[i + 2] == ' ' || chars[i + 2] == '(')
                        {
                            line_tracker.set_token_line(ret.len());
                            i += 1;
                            Some(Token::If)
                        } else {
                            None
                        }
                    }
                    'r' => {
                        if chars[i + 1] == 'e'
                            && chars[i + 2] == 't'
                            && chars[i + 3] == 'u'
                            && chars[i + 4] == 'r'
                            && chars[i + 5] == 'n'
                            && (chars[i + 6] == '(' || chars[i + 6] == ' ')
                        {
                            line_tracker.set_token_line(ret.len());
                            i += 5;
                            Some(Token::Return)
                        } else {
                            None
                        }
                    }
                    'c' => {
                        if chars[i + 1] == 'h'
                            && chars[i + 2] == 'a'
                            && chars[i + 3] == 'r'
                            && chars[i + 4] == ' '
                        {
                            line_tracker.set_token_line(ret.len());
                            i += 3;
                            Some(Token::Type(RhTypes::Char))
                        } else {
                            None
                        }
                    }
                    'f' => {
                        if chars[i + 1] == 'o' && chars[i + 2] == 'r' && chars[i + 3] == ' ' {
                            line_tracker.set_token_line(ret.len());
                            i += 2;
                            Some(Token::For)
                        } else {
                            None
                        }
                    }
                    'l' => {
                        if chars[i + 1] == 'o'
                            && chars[i + 2] == 'o'
                            && chars[i + 3] == 'p'
                            && chars[i + 4] == ' '
                        {
                            line_tracker.set_token_line(ret.len());
                            i += 3;
                            Some(Token::Loop)
                        } else {
                            None
                        }
                    }
                    'a' => {
                        if chars[i + 1] == 's'
                            && chars[i + 2] == 's'
                            && chars[i + 3] == 'e'
                            && chars[i + 4] == 'r'
                            && chars[i + 5] == 't'
                            && chars[i + 6] == ' '
                        {
                            line_tracker.set_token_line(ret.len());
                            i += 5;
                            Some(Token::Assert)
                        } else if chars[i + 1] == 's'
                            && chars[i + 2] == 'm'
                            && (chars[i + 3] == ' ' || chars[i + 3] == '(')
                        {
                            line_tracker.set_token_line(ret.len());
                            i += 2;
                            Some(Token::Asm)
                        } else {
                            None
                        }
                    }
                    'p' => {
                        if chars[i + 1] == 'u'
                            && chars[i + 2] == 't'
                            && (chars[i + 3] == '(' || chars[i + 3] == ' ')
                        {
                            line_tracker.set_token_line(ret.len());
                            i += 2;
                            Some(Token::PutChar)
                        } else {
                            None
                        }
                    }
                    's' => {
                        if chars[i + 1] == 't'
                            && chars[i + 2] == 'r'
                            && chars[i + 3] == 'u'
                            && chars[i + 4] == 'c'
                            && chars[i + 5] == 't'
                        {
                            line_tracker.set_token_line(ret.len());
                            i += 5;
                            Some(Token::Struct)
                        } else {
                            None
                        }
                    }

                    '-' => match chars[i + 1] {
                        '=' => {
                            line_tracker.set_token_line(ret.len());
                            i += 1;
                            Some(Token::SubEq)
                        }
                        '>' => {
                            line_tracker.set_token_line(ret.len());
                            i += 1;
                            Some(Token::Arrow)
                        }
                        '-' => {
                            line_tracker.set_token_line(ret.len());
                            i += 1;
                            Some(Token::SubO)
                        }
                        '0'..='9' => {
                            let mut num = String::from("-");
                            let radix = if chars[i] == '0' {
                                i += 1;
                                match chars[i] {
                                    'x' => 12,
                                    'o' => 8,
                                    'b' => 2,
                                    'a' | 'c'..='n' | 'p'..='w' | 'y' | 'z' => {
                                        panic!("Unsupported base")
                                    }
                                    _ => {
                                        num.push_str("0");
                                        10
                                    }
                                }
                            } else {
                                10
                            };

                            while chars[i].is_alphanumeric() {
                                num.push(chars[i]);
                                i += 1;
                            }

                            let value = i32::from_str_radix(&num, radix)
                                .expect("Invalid numeric literal. Probably a lexing error");

                            line_tracker.set_token_line(ret.len());
                            Some(Token::NumLiteral(value))
                        }
                        _ => {
                            line_tracker.set_token_line(ret.len());
                            Some(Token::Sub)
                        }
                    },

                    // obviously none of this can be included in ids
                    'L' => {
                        if chars[i + 1] == 'A'
                            && chars[i + 2] == 'B'
                            && chars[i + 3] == 'E'
                            && chars[i + 4] == 'L'
                            && chars[i + 5] == ':'
                        {
                            let mut name = String::new();
                            i += 6;
                            while chars[i].is_alphanumeric() {
                                name.push(chars[i]);
                                i += 1;
                            }
                            line_tracker.set_token_line(ret.len());

                            Some(Token::Label(name))
                        } else {
                            None
                        }
                    }
                    'g' => {
                        if chars[i + 1] == 'o' && chars[i + 2] == 't' && chars[i + 3] == 'o' {
                            let mut name = String::new();
                            i += 4;
                            while chars[i].is_alphanumeric() {
                                name.push(chars[i]);
                                i += 1;
                            }
                            line_tracker.set_token_line(ret.len());
                            Some(Token::Goto(name))
                        } else {
                            None
                        }
                    }
                    'w' => {
                        if chars[i + 1] == 'h'
                            && chars[i + 2] == 'i'
                            && chars[i + 3] == 'l'
                            && chars[i + 4] == 'e'
                        {
                            line_tracker.set_token_line(ret.len());
                            i += 4;
                            Some(Token::While)
                        } else {
                            None
                        }
                    }
                    'v' => {
                        if chars[i + 1] == 'o'
                            && chars[i + 2] == 'i'
                            && chars[i + 3] == 'd'
                            && (chars[i + 4] == ' ' || chars[i + 4] == '*')
                        {
                            line_tracker.set_token_line(ret.len());
                            i += 3;
                            Some(Token::Type(RhTypes::Void))
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if token.is_some() {
                    ret.push(token.unwrap());
                } else {
                    let mut name = String::new();
                    while chars[i].is_alphanumeric() || chars[i] == '_' {
                        name.push(chars[i]);
                        i += 1;
                    }
                    ret.push(Token::Id(name));
                    line_tracker.set_token_line(ret.len());
                }
            }
            '+' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '=' => {
                        i += 1;
                        Token::AddEq
                    }
                    '+' => {
                        line_tracker.set_token_line(ret.len());
                        i += 1;
                        Token::AddO
                    }
                    _ => {
                        line_tracker.set_token_line(ret.len());
                        Token::Add
                    }
                })
            }
            '/' => {
                let token = match chars[i + 1] {
                    '=' => {
                        line_tracker.set_token_line(ret.len());
                        i += 1;
                        Some(Token::DivEq)
                    }
                    '/' => {
                        i += 1;
                        while chars[i] != '\n' {
                            i += 1;
                        }
                        None
                    }
                    _ => {
                        line_tracker.set_token_line(ret.len());
                        Some(Token::Div)
                    }
                };
                if token.is_some() {
                    ret.push(token.unwrap());
                }
            }
            '*' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '*' => {
                        i += 1;
                        Token::MulEq
                    }
                    _ => Token::Star,
                });
            }
            '(' => {
                ret.push(Token::OParen);
                line_tracker.set_token_line(ret.len());
            }
            ')' => {
                ret.push(Token::CParen);
                line_tracker.set_token_line(ret.len());
            }
            '{' => {
                ret.push(Token::OCurl);
                line_tracker.set_token_line(ret.len());
            }
            '}' => {
                ret.push(Token::CCurl);
                line_tracker.set_token_line(ret.len());
            }
            '[' => {
                ret.push(Token::OSquare);
                line_tracker.set_token_line(ret.len());
            }
            ']' => {
                ret.push(Token::CSquare);
                line_tracker.set_token_line(ret.len());
            }
            '&' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '=' => {
                        i += 1;
                        Token::BAndEq
                    }
                    '&' => {
                        i += 1;
                        Token::AndCmp
                    }
                    _ => Token::BAnd,
                });
            }
            '^' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '=' => {
                        i += 1;
                        Token::BXorEq
                    }
                    _ => Token::BXor,
                });
            }
            '%' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '=' => {
                        i += 1;
                        Token::ModEq
                    }
                    _ => Token::Mod,
                });
            }
            '!' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '=' => {
                        i += 1;
                        Token::NeqCmp
                    }
                    _ => Token::Neq,
                });
            }
            '|' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '=' => {
                        i += 1;
                        Token::BOrEq
                    }
                    '|' => {
                        i += 1;
                        Token::OrCmp
                    }
                    _ => Token::BOr,
                });
            }
            '~' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '=' => {
                        i += 1;
                        Token::BNotEq
                    }
                    _ => Token::BNot,
                });
            }
            '<' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '=' => {
                        i += 1;
                        Token::LsEq
                    }
                    '<' => {
                        if chars[i + 2] == '=' {
                            Token::BLSEq
                        } else {
                            Token::BLS
                        }
                    }
                    _ => Token::Ls,
                });
            }
            '>' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '=' => {
                        i += 1;
                        Token::GrEq
                    }
                    '>' => {
                        if chars[i + 2] == '=' {
                            Token::BRSEq
                        } else {
                            Token::BRS
                        }
                    }
                    _ => Token::Gr,
                });
            }
            '.' => {
                line_tracker.set_token_line(ret.len());
                ret.push(Token::Dot);
            }
            ',' => {
                ret.push(Token::Comma);
                line_tracker.set_token_line(ret.len());
            }
            ';' => {
                ret.push(Token::Semi);
                line_tracker.set_token_line(ret.len());
            }
            '=' => {
                line_tracker.set_token_line(ret.len());
                ret.push(match chars[i + 1] {
                    '=' => {
                        i += 1;
                        Token::EqCmp
                    }
                    _ => Token::Eq,
                });
            }
            '\n' => {
                line_tracker.new_line();
            }
            '\'' => {
                if !chars[i + 1].is_ascii() {
                    continue;
                }

                i += 1;
                ret.push(Token::NumLiteral(if chars[i] == '\\' {
                    if chars[i + 1].is_ascii_digit() {
                        chars[i + 1].to_digit(10).expect("Invalid literal digit") as i32
                    } else {
                        match chars[i + 1] {
                            'n' => 10,
                            't' => 9,
                            _ => 0,
                        }
                    }
                } else {
                    chars[i + 1] as i32
                }));
                line_tracker.set_token_line(ret.len());
                i += 2;
            }
            _ => {
                panic!("How did we get here {}", chars[i]);
            }
        }
        i += 1;
    }
    Ok((ret, line_tracker))
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    If,
    Break,
    For,
    While,
    Loop,
    Fn,
    Type(RhTypes),
    Struct,
    Star,
    NumLiteral(i32),
    StrLiteral(String),
    Add,
    AddO,
    Sub,
    SubO,
    Div,
    Mod,
    ModEq,
    Eq,
    Id(String), // why is there id and var???
    EqCmp,
    NeqCmp,
    AndCmp,
    OrCmp,
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
    BLS,
    BLSU,
    BLSEq,
    BRS,
    BRSU,
    BRSEq,
    OParen,
    CParen,
    OCurl,
    CCurl,
    OSquare,
    CSquare,
    Goto(String),
    Label(String),
    Asm,
    Dot,
    Comma,
    Semi,
    Arrow,
    Return,
    PutChar,
    Assert, // this might be to much for the lexer to do
            // FuncDeclare((String, Vec<String>, RhTypes)), // function name, args, return type
            // FuncCall(String, Vec<String>), // function name, args
}

#[derive(Debug, PartialEq, Clone)]
pub enum RhTypes {
    Char,
    Int,
    Void,
}
