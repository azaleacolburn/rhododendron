use crate::lexer::Token;

// Valid Node Types
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Program,
    Sub,
    Div,
    Eq,
    Id(String), // figure out if we want this here
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
    Loop,
    Assignment(Option<String>),
    Declaration(Option<String>)
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
    pub children: Option<Vec<TokenNode>>
}

impl std::fmt::Display for TokenNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type: {:?}", self.token) // doesn't print values
    }
}

impl TokenNode {
    pub fn new(token: NodeType, children: Option<Vec<TokenNode>>) -> TokenNode {
        TokenNode { token, children }
    }

    pub fn print(&self, n: &mut i32) {
        println!("Token: {}", self);
        println!("Children: ");
        for _i in 0..*n {
            print!("    ");
        }
        *n += 1;
        if self.children.is_some() {
            for node in self.children.as_ref().expect("Children is Some") {
                node.print(n);
            }
        }
        *n -= 1;
        // println!("End Children");
    }
}

#[derive(Debug)]
pub enum Error {
    ExpectedCParen,
    ExpectedExpression,
    ExpectedId,
    UndeclaredId,
    ExpectedAssignment,
    ExpectedStatement
}

#[derive(Debug)]
pub struct RhErr {
    err: Error,
    token_i: Option<usize>
}

impl RhErr {
    pub fn new(err: Error, token_i: Option<usize>) -> RhErr {
        RhErr { err, token_i }
    }
}

pub fn program(tokens: &Vec<Token>) -> Result<TokenNode, RhErr> {
    let mut token_i: &mut usize = &mut 0;
    let mut program_node = TokenNode::new(NodeType::Program, Some(vec![]));
    while tokens.len() > *token_i + 1 {
        match statement(tokens, token_i) {
            Ok(node) => {
                program_node.children.as_mut().expect("a valid ast to be returned").push(node);
            },
            Err(err) => return Err(err),
        };
        *token_i += 1;
    }
    println!("past parsing");
    Ok(program_node)
}

pub fn statement(tokens: &Vec<Token>, token_i: &mut usize) -> Result<TokenNode, RhErr> {
    // let mut node: TokenNode = TokenNode::new(NodeType::Program, Some(vec![])); // todo: add default type
    println!("statment token: {:?}", &tokens[*token_i]);
    match &tokens[*token_i] {
        Token::Type(_) => declaration(tokens, token_i),
        Token::Id(name) => assignment(tokens, token_i, name.to_string()),
        _ => Err(RhErr::new(Error::ExpectedStatement, None))
    }
}

fn declaration(tokens: &Vec<Token>, token_i: &mut usize) -> Result<TokenNode, RhErr> {
    let mut node = TokenNode::new(NodeType::Declaration(None), Some(vec![]));
    *token_i += 1;
    match &tokens[*token_i] {
        Token::Id(id) => {
            node.token = NodeType::Declaration(Some(id.to_string()));
            *token_i += 1;
            if tokens[*token_i] == Token::Eq {
                node.children.as_mut().expect("node to have children").push(
                    match expression(tokens, token_i) {
                        Ok(node) => node,
                        Err(err) => return Err(err)
                    }
                );
            } else if tokens[*token_i] == Token::Semi {
                return Ok(node.clone());
            }
        },
        _ => return Err(RhErr::new(Error::ExpectedId, Some(*token_i)))
    };
    Ok(node.clone())
}

fn expression(tokens: &Vec<Token>, token_i: &mut usize) -> Result<TokenNode, RhErr> {
    let mut left = match term(tokens, token_i) {
        Ok(node) => node,
        Err(err) => return Err(err)
    };

    // *token_i += 1;
    let mut curr = &tokens[*token_i];
    while *curr == Token::Add || *curr == Token::Sub {
        let op: &mut Option<Token> = &mut None;
        *op = Some(curr.clone());

        let right = match term(tokens, token_i) {
            Ok(node) => node,
            Err(err) => return Err(err)
        };
        let op_tok = TokenNode::new(NodeType::from_token(op.as_ref().expect("Op to have a value")).unwrap(), Some(vec![left, right]));

        left = op_tok;
        // *token_i += 1;
        curr = &tokens[*token_i];
        println!("{:?}", curr);
    }
    Ok(left)
}

fn term(tokens: &Vec<Token>, token_i: &mut usize) -> Result<TokenNode, RhErr> {
    println!("term");
    let mut left: TokenNode = match factor(tokens, token_i) {
        Ok(node) => node,
        Err(err) => return Err(err)
    };
    *token_i += 1;
    let mut curr = &tokens[*token_i];
    while *curr == Token::Star || *curr == Token::Div {
        println!("in term loop(should only happen once)");
        let op = &mut Token::Add;
        *op = curr.clone();
        let right = match factor(tokens, token_i) {
            Ok(node) => node,
            Err(err) => return Err(err)
        };
        println!("op: {:?}", op);
        let op_tok = TokenNode::new(NodeType::from_token(op).unwrap(), Some(vec![left, right]));
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
        Token::NumLiteral(num) => Ok(TokenNode::new(NodeType::NumLiteral(*num), None)),
        Token::Id(id) => Ok(TokenNode::new(NodeType::Id(id.to_string()), None)),
        Token::OParen => {
            *token_i += 1;
            match expression(tokens, token_i) {
                Ok(node) => {
                    if tokens[*token_i] == Token::CParen { Ok(node) } 
                    else { Err(RhErr::new(Error::ExpectedCParen, Some(*token_i))) }
                },
                Err(err) => Err(err)
            }
        },
        _ => Err(RhErr::new(Error::ExpectedExpression, Some(*token_i)))
    }
}

fn assignment(tokens: &Vec<Token>, token_i: &mut usize, name: String) -> Result<TokenNode, RhErr> {
    Ok(
        TokenNode::new(NodeType::Assignment(Some(name)), Some(vec![
            TokenNode::new(NodeType::from_token(&tokens[*token_i]).expect("valid op token"), Some(vec![
                expression(tokens, token_i).expect("valid expression")
            ]))
        ]))
    )
}