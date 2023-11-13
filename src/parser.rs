use crate::lexer::{
    Token,
};

// Valid Node Types
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Program,
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
    Mul,
    NumLiteral(i32),
    Add,
    If,
    For,
    While,
    Assignment,
    Declaration
}

impl NodeType {
    fn from_token(tok: &Token) -> Result<NodeType, ()> {
        println!("tok: {:?}", tok);
        match tok {
            Token::Sub => Ok(NodeType::Sub),
            Token::Div => Ok(NodeType::Div),
            Token::Eq => Ok(NodeType::Eq),
            Token::Id(str) => Ok(NodeType::Id(str.to_string())),
            Token::EqCmp => Ok(NodeType::EqCmp),
            Token::NeqCmp => Ok(NodeType::NeqCmp),
            Token::BOr => Ok(NodeType::BOr),
            Token::BAnd => Ok(NodeType::BAnd),
            Token::BXor => Ok(NodeType::BXor),
            Token::BOrEq => Ok(NodeType::BOrEq),
            Token::BAndEq => Ok(NodeType::BAndEq),
            Token::BXorEq => Ok(NodeType::BXorEq),
            Token::SubEq => Ok(NodeType::SubEq),
            Token::AddEq => Ok(NodeType::AddEq),
            Token::DivEq => Ok(NodeType::DivEq),
            Token::MulEq => Ok(NodeType::MulEq),
            Token::Star => Ok(NodeType::Mul), // exception for pointer
            Token::NumLiteral(i) => Ok(NodeType::NumLiteral(*i)),
            Token::Add => Ok(NodeType::Add),
            Token::For => Ok(NodeType::For),
            Token::While => Ok(NodeType::While),
            Token::If => Ok(NodeType::If),
            _ => { println!("Oh God No, Not A Valid Token"); return Err(())}
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenNode {
    pub token: NodeType,
    pub children: Vec<TokenNode>
}

impl std::fmt::Display for TokenNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type: {:?}", self.token) // doesn't print values
    }
}

impl TokenNode {
    pub fn new(token: NodeType, children: Vec<TokenNode>) -> TokenNode {
        TokenNode { token, children }
    }

    pub fn print(&self) {
        println!("Token: {}", self);
        println!("Children: ");
        for node in &self.children {
            node.print();
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ExpectedCParen,
    ExpectedExpression
}

#[derive(Debug)]
pub struct RhErr {
    err: Error,
    token_i: usize
}

impl RhErr {
    fn new(err: Error, token_i: usize) -> RhErr {
        RhErr { err, token_i }
    }
}

pub fn program(tokens: &Vec<Token>) -> Result<TokenNode, RhErr> {
    let mut node: TokenNode = TokenNode::new(NodeType::Program, vec![]); // todo: add default type
    let mut token_i: usize = 0;
    match &tokens[token_i] {
        Token::Type(_) => { node.children.push(declare(tokens, &mut token_i).unwrap().clone()); },
        Token::Id(_) => {

        },
        _ => {}
    }
    Ok(node)
}

fn declare(tokens: &Vec<Token>, token_i: &mut usize) -> Result<TokenNode, RhErr> {
    let mut node = TokenNode::new(NodeType::Declaration, vec![]);
    *token_i += 1;
    match &tokens[*token_i] {
        Token::Id(id) => {
            *token_i += 1;
            if tokens[*token_i] == Token::Eq {
                node.children.push(
                    match expr(tokens, token_i) {
                        Ok(node) => {
                            node
                        },
                        Err(err) => { return Err(err); }
                    }
                );
            } else if tokens[*token_i] == Token::Semi {
                // next statement
            }
        },
        _ => {

        }
    }
    Ok(node.clone())
}

fn expr(tokens: &Vec<Token>, token_i: &mut usize) -> Result<TokenNode, RhErr> {
    let mut left = match term(tokens, token_i) {
        Ok(node) => {
            node
        },
        Err(err) => return Err(err)
    };

    // *token_i += 1;
    let mut curr = &tokens[*token_i];
    while *curr == Token::Star || *curr == Token::Div {
        let op = &mut Token::Add;
        *op = curr.clone();

        let right = match term(tokens, token_i) {
            Ok(node) => {
                node
            },
            Err(err) => return Err(err)
        };
        let op_tok = TokenNode::new(NodeType::from_token(op).unwrap(), vec![left, right]);

        left = op_tok;
        // *token_i += 1;
        curr = &tokens[*token_i];
    }
    Ok(left)
}

fn term(tokens: &Vec<Token>, token_i: &mut usize) -> Result<TokenNode, RhErr> {
    println!("term");
    let mut left = match factor(tokens, token_i) {
        Ok(node) => {
            node
        },
        Err(err) => return Err(err)
    };
    *token_i += 1;
    let mut curr = &tokens[*token_i];
    while *curr == Token::Add || *curr == Token::Sub {
        println!("in term loop(should only happen once)");
        let op = &mut Token::Add;
        *op = curr.clone();
        let right = match factor(tokens, token_i) {
            Ok(node) => {
                node
            },
            Err(err) => return Err(err)
        };
        println!("op: {:?}", op);
        let op_tok = TokenNode::new(NodeType::from_token(op).unwrap(), vec![left, right]);
        left = op_tok;
        *token_i += 1;
        curr = &tokens[*token_i];
        println!("curr: {:?}", curr);
    }
    Ok(left)
}

fn factor(tokens: &Vec<Token>, token_i: &mut usize) -> Result<TokenNode, RhErr> {
    println!("factor");
    println!("{:?}", tokens[*token_i]);
    *token_i += 1;
    println!("{:?}", tokens[*token_i]);
    match &tokens[*token_i] {
        Token::NumLiteral(num) => {
            Ok(TokenNode::new(NodeType::NumLiteral(*num), vec![]))
        },
        Token::Id(id) => {
            Ok(TokenNode::new(NodeType::Id(id.to_string()), vec![]))
        },
        Token::OParen => {
            *token_i += 1;
            match expr(tokens, token_i) {
                Ok(node) => {
                    if tokens[*token_i] == Token::OParen {
                        Ok(node)
                    } else {
                        Err(RhErr::new(Error::ExpectedCParen, *token_i)) // no closing parenthesis
                    }
                },
                Err(err) => return Err(err)
            }
        },
        _ => {
            Err(RhErr::new(Error::ExpectedExpression, *token_i))
        }
    }
}