use crate::lexer::{RhTypes, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum ScopeType {
    Function,
    While,
    Program,
    If,
    Loop,
    For,
}

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
    AndCmp,
    OrCmp,
    NumLiteral(i32),
    Add,
    If,
    For,
    While,
    Loop,
    Break,
    FunctionCall(String),
    Scope(Option<RhTypes>), // <-- anything that has {} is a scope, scope is how we're handling multiple statements, scopes return the last statement's result or void
    Condition(bool), // true is eq false is neq; This might not be completely clear when optimizing conditionals and loops start
    Assignment(Option<String>),
    Declaration(Option<String>),
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
            Token::Break => Ok(NodeType::Break),
            _ => {
                println!("Oh God No, Not A Valid Token");
                return Err(());
            }
        }
    }
}

pub struct TokenHandler {
    tokens: Vec<Token>,
    curr_token: usize,
}

impl TokenHandler {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenHandler {
            tokens: tokens,
            curr_token: 0,
        }
    }

    pub fn next_token(&mut self) {
        self.curr_token += 1;
    }

    pub fn prev_token(&mut self) {
        self.curr_token -= 1;
    }

    pub fn get_token(&self) -> &Token {
        &self.tokens[self.curr_token]
    }

    pub fn get_prev_token(&self) -> &Token {
        &self.tokens[self.curr_token - 1]
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

#[derive(Debug, Clone)]
pub struct TokenNode {
    pub token: NodeType,
    pub children: Option<Vec<TokenNode>>,
}

impl std::fmt::Display for TokenNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.token) // doesn't print values
    }
}

impl TokenNode {
    pub fn new(token: NodeType, children: Option<Vec<TokenNode>>) -> TokenNode {
        TokenNode { token, children }
    }

    pub fn print(&self, n: &mut i32) {
        (0..*n).into_iter().for_each(|_| {
            print!("    ");
        });
        println!("{}", self);
        *n += 1;
        if let Some(children) = &self.children {
            children.iter().for_each(|node| {
                node.print(n);
            })
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
    ExpectedStatement,
    ExpectedCondition,
    ExpectedOParen,
    ExpectedCCurl,
}

#[derive(Debug)]
pub struct RhErr {
    err: Error,
    token_i: Option<usize>,
}

impl RhErr {
    pub fn new(err: Error, token_i: Option<usize>) -> RhErr {
        RhErr { err, token_i }
    }
}

pub fn program(tokens: Vec<Token>) -> Result<TokenNode, RhErr> {
    let mut token_handler = TokenHandler::new(tokens);

    let mut program_node = TokenNode::new(NodeType::Program, Some(vec![]));
    let top_scope = match scope(&mut token_handler, ScopeType::Program) {
        Ok(node) => node,
        Err(err) => return Err(err),
    };
    program_node.children.as_mut().unwrap().push(top_scope);

    println!("past parsing");
    Ok(program_node)
}

pub fn scope(token_handler: &mut TokenHandler, scope_type: ScopeType) -> Result<TokenNode, RhErr> {
    let mut scope_node = TokenNode::new(NodeType::Scope(None), Some(vec![]));
    while *token_handler.get_token() != Token::CCurl {
        if token_handler.curr_token > token_handler.len() {
            return Err(RhErr::new(
                Error::ExpectedCCurl,
                Some(token_handler.curr_token),
            ));
        }

        match statement(token_handler, scope_type.clone()) {
            Ok(node) => {
                scope_node
                    .children
                    .as_mut()
                    .expect("a valid ast to be returned")
                    .push(node);
            }
            Err(err) => return Err(err),
        };

        println!("past if maybe\n");
        println!("past if token: {:?}", token_handler.get_token());
        if token_handler.curr_token == token_handler.len() - 1 {
            return Ok(scope_node);
        }
        token_handler.next_token();
        // println!("here\n");
        // if token_handler.len() == token_handler.curr_token + 1 {
        // if *token_handler.get_token() != Token::Semi {
        // scope_node.token = NodeType::Scope(Some(RhTypes::Int)) // TODO: Chane this to evaluate the type of the last statement
        // }
        // if *token_handler.get_token() == Token::CCurl { break; }
        // }
    }
    if *token_handler.get_prev_token() == Token::Semi {
        scope_node.token = NodeType::Scope(Some(RhTypes::Int)) // TODO: Change this to evaluate the  type of the last statement
    }
    println!("past scope\n");
    Ok(scope_node)
}

pub fn statement(
    token_handler: &mut TokenHandler,
    scope_type: ScopeType,
) -> Result<TokenNode, RhErr> {
    // let mut node: TokenNode = TokenNode::new(NodeType::Program, Some(vec![])); // todo: add default type
    let statement_token = token_handler.get_token();
    println!("statment token: {:?}", statement_token);
    match statement_token {
        Token::Type(_) => declaration(token_handler),
        Token::Id(name) => assignment(token_handler, name.to_string()),
        Token::If => if_statement(token_handler),
        Token::While => while_statement(token_handler),
        Token::For => for_statement(token_handler),
        Token::Break => {
            if scope_type == ScopeType::While || scope_type == ScopeType::Loop {
                Ok(TokenNode::new(NodeType::Break, None))
            } else {
                Err(RhErr::new(
                    Error::ExpectedStatement,
                    Some(token_handler.curr_token),
                ))
            }
        }
        _ => Err(RhErr::new(Error::ExpectedStatement, None)),
    }
}

fn declaration(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut node = TokenNode::new(NodeType::Declaration(None), Some(vec![]));
    token_handler.next_token();
    match token_handler.get_token() {
        Token::Id(id) => {
            node.token = NodeType::Declaration(Some(id.to_string()));
            token_handler.next_token();
            if *token_handler.get_token() == Token::Eq {
                node.children.as_mut().expect("node to have children").push(
                    match expression(token_handler) {
                        Ok(node) => node,
                        Err(err) => return Err(err),
                    },
                );
            } else if *token_handler.get_token() == Token::Semi {
                return Ok(node.clone());
            }
        }
        _ => {
            return Err(RhErr::new(
                Error::ExpectedId,
                Some(token_handler.curr_token),
            ))
        }
    };
    Ok(node.clone())
}

fn expression(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut left = match term(token_handler) {
        Ok(node) => node,
        Err(err) => return Err(err),
    };

    // token_handler.next_token();
    let mut curr = token_handler.get_token();
    while *curr == Token::Add || *curr == Token::Sub {
        let op: &mut Option<Token> = &mut None;
        *op = Some(curr.clone());

        let right = match term(token_handler) {
            Ok(node) => node,
            Err(err) => return Err(err),
        };
        let op_tok = TokenNode::new(
            NodeType::from_token(op.as_ref().expect("Op to have a value")).unwrap(),
            Some(vec![left, right]),
        );

        left = op_tok;
        // token_handler.next_token();
        curr = &token_handler.get_token();
        println!("{:?}", curr);
    }
    Ok(left)
}

fn term(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    println!("term");
    let mut left: TokenNode = match factor(token_handler) {
        Ok(node) => node,
        Err(err) => return Err(err),
    };
    token_handler.next_token();
    let mut curr = token_handler.get_token();
    while *curr == Token::Star || *curr == Token::Div {
        println!("in term loop(should only happen once)");
        let op = &mut Token::Add;
        *op = curr.clone();
        let right = match factor(token_handler) {
            Ok(node) => node,
            Err(err) => return Err(err),
        };
        println!("op: {:?}", op);
        let op_tok = TokenNode::new(NodeType::from_token(op).unwrap(), Some(vec![left, right]));
        left = op_tok;
        token_handler.next_token();
        curr = &token_handler.get_token();
        println!("curr: {:?}", curr);
    }
    Ok(left)
}

fn factor(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    println!("factor");
    println!("before {:?}", token_handler.get_token());
    token_handler.next_token();
    println!("actual factor token {:?}", token_handler.get_token());
    match &token_handler.get_token() {
        Token::NumLiteral(num) => Ok(TokenNode::new(NodeType::NumLiteral(*num), None)),
        Token::Id(id) => Ok(TokenNode::new(NodeType::Id(id.to_string()), None)),
        Token::OParen => {
            token_handler.next_token();
            match expression(token_handler) {
                Ok(node) => {
                    if *token_handler.get_token() == Token::CParen {
                        Ok(node)
                    } else {
                        Err(RhErr::new(
                            Error::ExpectedCParen,
                            Some(token_handler.curr_token),
                        ))
                    }
                }
                Err(err) => Err(err),
            }
        }
        _ => Err(RhErr::new(
            Error::ExpectedExpression,
            Some(token_handler.curr_token),
        )),
    }
}

fn assignment(token_handler: &mut TokenHandler, name: String) -> Result<TokenNode, RhErr> {
    token_handler.next_token();
    // println!("ASSIGNMENT TOKEN: {:?}", token_handler.get_token());
    // println!("STARTED ASSIGNMENT TOKEN TOKEN TOKEN TOKEN");
    Ok(TokenNode::new(
        NodeType::Assignment(Some(name)),
        Some(vec![
            TokenNode::new(
                NodeType::from_token(token_handler.get_token()).expect("valid id"),
                None,
            ),
            expression(token_handler).expect("expression to be valid"),
        ]),
    ))
}

fn while_statement(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut while_node = TokenNode::new(NodeType::While, Some(vec![]));
    token_handler.next_token();
    let condition_node = condition(token_handler)?;
    while_node
        .children
        .as_mut()
        .expect("While children to be some")
        .push(condition_node);

    token_handler.next_token();
    token_handler.next_token();

    let scope_node = scope(token_handler, ScopeType::While)?;
    while_node
        .children
        .as_mut()
        .expect("While children to be ssome")
        .push(scope_node);
    Ok(while_node)
}

fn if_statement(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut if_node = TokenNode::new(NodeType::If, Some(vec![]));
    token_handler.next_token(); // might make semi handled by the called functions instead
    let condition_node = condition(token_handler)?;
    if_node
        .children
        .as_mut()
        .expect("If children to be some")
        .push(condition_node);

    token_handler.next_token();
    token_handler.next_token();

    let scope_node = scope(token_handler, ScopeType::If)?;
    if_node
        .children
        .as_mut()
        .expect("children to be some")
        .push(scope_node);
    Ok(if_node)
}

fn condition(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    // let condition_node = TokenNode::new(NodeType::Condition());
    // token_handler.next_token();
    println!("opening condition token: {:?}", token_handler.get_token());
    match token_handler.get_token() {
        Token::OParen => {
            // evaluate condition
            let condition = condition_expr(token_handler);
            match token_handler.get_token() {
                Token::CParen => condition,
                _ => {
                    println!("post condition {:?}", token_handler.get_token());
                    Err(RhErr::new(
                        Error::ExpectedCParen,
                        Some(token_handler.curr_token),
                    ))
                }
            }
        }
        _ => Err(RhErr::new(
            Error::ExpectedOParen,
            Some(token_handler.curr_token),
        )),
    }
}
/// This is called if expr needs parenthesis
fn condition_expr(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut left = match condition_term(token_handler) {
        Ok(node) => node,
        Err(err) => return Err(err),
    };

    let mut curr = token_handler.get_token();
    while *curr == Token::AndCmp || *curr == Token::OrCmp {
        // && ||
        let cmp: &mut Option<Token> = &mut None;
        *cmp = Some(curr.clone());

        let right = match condition_term(token_handler) {
            Ok(node) => node,
            Err(err) => return Err(err),
        };
        let cmp_tok = TokenNode::new(
            NodeType::from_token(cmp.as_ref().expect("Op to have a value")).unwrap(),
            Some(vec![left, right]),
        );

        left = cmp_tok;
        // token_handler.next_token();
        curr = &token_handler.get_token();
        println!("{:?}", curr);
    }
    Ok(left)
}

fn condition_term(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut left = match condition_factor(token_handler) {
        Ok(node) => node,
        Err(err) => return Err(err),
    };
    token_handler.next_token();
    let mut curr = token_handler.get_token();
    while *curr == Token::NeqCmp || *curr == Token::EqCmp {
        // != ==
        let cmp: &mut Option<Token> = &mut None;
        *cmp = Some(curr.clone());
        println!("condition cmp: {:?}", cmp);
        let right = match condition_factor(token_handler) {
            Ok(node) => node,
            Err(err) => return Err(err),
        };
        let cmp_tok = TokenNode::new(
            NodeType::from_token(cmp.as_ref().expect("Op to have a value")).unwrap(),
            Some(vec![left, right]),
        );

        left = cmp_tok;
        token_handler.next_token();
        curr = &token_handler.get_token();
        println!("{:?}", curr);
    }
    Ok(left)
}

fn condition_factor(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    token_handler.next_token();
    println!("factor: {:?}", token_handler.get_token());
    match &token_handler.get_token() {
        Token::NumLiteral(num) => Ok(TokenNode::new(NodeType::NumLiteral(*num), None)),
        Token::Id(name) => Ok(TokenNode::new(NodeType::Id(name.clone()), None)),
        Token::OParen => {
            token_handler.next_token();
            match expression(token_handler) {
                Ok(node) => {
                    if *token_handler.get_token() == Token::CParen {
                        Ok(node)
                    } else {
                        Err(RhErr::new(
                            Error::ExpectedCParen,
                            Some(token_handler.curr_token),
                        ))
                    }
                }
                Err(err) => Err(err),
            }
        }
        _ => {
            return Err(RhErr::new(
                Error::ExpectedCondition,
                Some(token_handler.curr_token),
            ))
        }
    }
}

fn for_statement(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut for_node = TokenNode::new(NodeType::If, Some(vec![]));
    let declare_node = match declaration(token_handler) {
        Ok(node) => node,
        Err(err) => return Err(err),
    };
    for_node
        .children
        .as_mut()
        .expect("vec to be some")
        .push(declare_node);
    println!("token: {:?}, should be ;", token_handler.get_token());
    token_handler.next_token(); // might make semi handled by the called functions instead
    let condition_node = match condition(token_handler) {
        Ok(node) => node,
        Err(err) => return Err(err),
    };
    for_node
        .children
        .as_mut()
        .expect("vec to be some")
        .push(condition_node);
    token_handler.next_token();
    let statement_node = match statement(token_handler, ScopeType::For) {
        Ok(node) => node,
        Err(err) => return Err(err),
    };
    for_node
        .children
        .as_mut()
        .expect("vec to be some")
        .push(statement_node);
    token_handler.next_token();
    Ok(for_node)
}
