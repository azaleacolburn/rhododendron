use crate::{
    compiler::{
        error::{RhErr, ET},
        lexer::{LineNumHandler, RhTypes, Token},
        DEBUG,
    },
    dbg_println,
};

#[derive(Debug, PartialEq, Clone)]
pub enum ScopeType {
    Function(RhTypes),
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
    MNeg,
    AndCmp,
    OrCmp,
    NumLiteral(i32),
    Add,
    If,
    For,
    While,
    _Loop,
    Break,
    FunctionCall(String),
    Scope(Option<RhTypes>), // <-- anything that has {} is a scope, scope is how we're handling multiple statements, scopes return the last statement's result or void
    Assignment(AssignmentOpType),
    Declaration((String, RhTypes)),
    Asm(String),
    Adr(String),
    DeRef,
    Array(i32),
    FunctionDeclaration((String, RhTypes)),
    Type(RhTypes),
    Assert,
    Return,
    PutChar,
    StructDeclaration(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOpType {
    Eq,
    SubEq,
    AddEq,
    DivEq,
    MulEq,
    BOrEq,
    BAndEq,
    BXorEq,
}

impl AssignmentOpType {
    fn from_token(tok: &Token) -> Result<AssignmentOpType, ()> {
        match tok {
            Token::Eq => Ok(AssignmentOpType::Eq),
            Token::SubEq => Ok(AssignmentOpType::SubEq),
            Token::AddEq => Ok(AssignmentOpType::AddEq),
            Token::DivEq => Ok(AssignmentOpType::DivEq),
            Token::MulEq => Ok(AssignmentOpType::MulEq),
            Token::BOrEq => Ok(AssignmentOpType::BOrEq),
            Token::BAndEq => Ok(AssignmentOpType::BAndEq),
            Token::BXorEq => Ok(AssignmentOpType::BXorEq),
            _ => {
                println!("Oh God No, Not A Valid OpEq Token");
                return Err(());
            }
        }
    }
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
            Token::OrCmp => Ok(NodeType::OrCmp),
            Token::AndCmp => Ok(NodeType::AndCmp),
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
    line_handler: LineNumHandler,
}

#[allow(dead_code)]
impl TokenHandler {
    pub fn new(tokens: Vec<Token>, line_handler: LineNumHandler) -> Self {
        TokenHandler {
            tokens,
            curr_token: 0,
            line_handler,
        }
    }

    pub fn next_token(&mut self) {
        self.curr_token += 1;
    }

    pub fn peek(&self, i: usize) -> &Token {
        &self.tokens[self.curr_token + i]
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

    pub fn new_err(&self, err: ET) -> RhErr {
        RhErr {
            err,
            line: self.line_handler.get_token_line(self.curr_token),
        }
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
        (0..*n).into_iter().for_each(|_| print!("  "));
        dbg_println!("{}", self);
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

pub fn program(tokens: Vec<Token>, line_tracker: LineNumHandler) -> Result<TokenNode, RhErr> {
    println!("test");
    let mut token_handler = TokenHandler::new(tokens, line_tracker);

    let mut program_node = TokenNode::new(NodeType::Program, Some(vec![]));
    let top_scope = scope(&mut token_handler, ScopeType::Program)?;
    program_node.children.as_mut().unwrap().push(top_scope);

    program_node.print(&mut 0);
    Ok(program_node)
}

pub fn scope(token_handler: &mut TokenHandler, scope_type: ScopeType) -> Result<TokenNode, RhErr> {
    println!("scope");
    let mut scope_node = TokenNode::new(NodeType::Scope(None), Some(vec![]));
    while *token_handler.get_token() != Token::CCurl {
        if token_handler.curr_token > token_handler.len() {
            return Err(token_handler.new_err(ET::ExpectedCParen));
        }

        scope_node
            .children
            .as_mut()
            .expect("Scope has no children")
            .push(statement(token_handler, scope_type.clone())?);
        dbg_println!();
        if token_handler.curr_token == token_handler.len() - 1 {
            return Ok(scope_node);
        }
        token_handler.next_token();
    }
    if *token_handler.get_prev_token() == Token::Semi {
        scope_node.token = NodeType::Scope(Some(RhTypes::Int)) // TODO: Change this to evaluate the  type of the last statement
    }
    Ok(scope_node)
}

pub fn statement(
    token_handler: &mut TokenHandler,
    scope_type: ScopeType,
) -> Result<TokenNode, RhErr> {
    // let mut node: TokenNode = TokenNode::new(NodeType::Program, Some(vec![])); // todo: add default type
    let statement_token = token_handler.get_token();
    dbg_println!("Statement Token: {:?}", statement_token);
    match statement_token {
        Token::Type(t) => type_statement(token_handler, t.clone()),
        Token::Id(name) => id_statement(token_handler, name.to_string()),
        // TODO: Maybe split deref_assignment into two null-terminals
        Token::Star => {
            token_handler.next_token();
            deref_assignment(token_handler, None)
        }
        Token::If => if_statement(token_handler),
        Token::While => while_statement(token_handler),
        Token::For => for_statement(token_handler),
        Token::Break => {
            if scope_type == ScopeType::While || scope_type == ScopeType::Loop {
                Ok(TokenNode::new(NodeType::Break, None))
            } else {
                Err(token_handler.new_err(ET::ExpectedStatement))
            }
        }
        Token::Asm => asm_statement(token_handler),
        Token::Assert => assert_statement(token_handler),
        Token::Return => return_statement(token_handler),
        Token::PutChar => putchar_statement(token_handler),
        //Token::Struct => struct_statement(token_handler),
        _ => Err(token_handler.new_err(ET::ExpectedStatement)),
    }
}

fn declaration(token_handler: &mut TokenHandler, t: RhTypes) -> Result<TokenNode, RhErr> {
    token_handler.next_token();
    match token_handler.get_token() {
        Token::Id(id) => {
            let mut node = TokenNode::new(NodeType::Declaration((id.to_string(), t)), Some(vec![]));
            token_handler.next_token();
            if *token_handler.get_token() == Token::Eq {
                token_handler.next_token();
                node.children
                    .as_mut()
                    .expect("node to have children")
                    .push(condition_expr(token_handler)?);
            }
            Ok(node.clone())
        }
        _ => Err(token_handler.new_err(ET::ExpectedId)),
    }
}

fn declaration_statement(token_handler: &mut TokenHandler, t: RhTypes) -> Result<TokenNode, RhErr> {
    let declare = declaration(token_handler, t);
    if *token_handler.get_token() != Token::Semi {
        return Err(token_handler.new_err(ET::ExpectedSemi));
    }
    declare
}

fn arithmetic_expression(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut left = arithmetic_term(token_handler)?;
    let mut curr = token_handler.get_token().clone();
    dbg_println!("Expression curr: {:?}", curr);
    while curr == Token::Add || curr == Token::Sub {
        token_handler.next_token();
        let right = arithmetic_term(token_handler)?;
        left = TokenNode::new(
            NodeType::from_token(&curr).unwrap(),
            Some(vec![left, right]),
        );
        curr = token_handler.get_token().clone();
    }
    Ok(left)
}

fn arithmetic_term(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut left: TokenNode = arithmetic_factor(token_handler)?;
    let mut curr = token_handler.get_token().clone();
    dbg_println!("Term curr: {:?}", curr);
    while curr == Token::Star || curr == Token::Div {
        token_handler.next_token();
        let right = arithmetic_factor(token_handler)?;
        left = TokenNode::new(
            NodeType::from_token(&curr).unwrap(),
            Some(vec![left, right]),
        );
        curr = token_handler.get_token().clone();
    }
    Ok(left)
}

fn arithmetic_factor(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let token = token_handler.get_token().clone();
    let ret = match token {
        Token::NumLiteral(num) => Ok(TokenNode::new(NodeType::NumLiteral(num), None)),
        Token::Id(id) => {
            if *token_handler.peek(1) == Token::OParen {
                Ok(function_call(token_handler, id.to_string())?)
            } else if *token_handler.peek(1) == Token::OSquare {
                token_handler.next_token();
                token_handler.next_token();
                let post_mul = TokenNode::new(
                    NodeType::Mul,
                    vec![
                        arithmetic_expression(token_handler)?,
                        TokenNode::new(NodeType::NumLiteral(8), None),
                    ]
                    .into(),
                );
                let post_add = TokenNode::new(
                    NodeType::Sub,
                    vec![TokenNode::new(NodeType::Id(id.to_string()), None), post_mul].into(),
                );
                if *token_handler.get_token() != Token::CSquare {
                    return Err(token_handler.new_err(ET::ExpectedCSquare));
                }
                Ok(TokenNode::new(NodeType::DeRef, vec![post_add].into()))
            } else {
                Ok(TokenNode::new(NodeType::Id(id.to_string()), None))
            }
        }

        // Address of a variable
        Token::BAnd => {
            token_handler.next_token();
            if let Token::Id(id) = token_handler.get_token() {
                Ok(TokenNode::new(NodeType::Adr(id.to_string()), None))
            } else {
                Err(token_handler.new_err(ET::ExpectedId))
            }
        }

        Token::Star => {
            token_handler.next_token();
            let factor = arithmetic_factor(token_handler)?;
            token_handler.prev_token();
            Ok(TokenNode::new(NodeType::DeRef, vec![factor].into()))
        }

        Token::OSquare => {
            token_handler.next_token();

            let n = if let Token::NumLiteral(n) = token_handler.get_token() {
                n
            } else {
                panic!("No empty arrays allowed");
            };

            let mut node = TokenNode::new(NodeType::Array(*n), vec![].into());

            dbg_println!("Array: {:?}", token_handler.get_token());

            token_handler.next_token();

            if *token_handler.get_token() == Token::Semi {
                token_handler.next_token();
                loop {
                    node.children
                        .as_mut()
                        .unwrap()
                        .push(condition_expr(token_handler)?);
                    if *token_handler.get_token() != Token::Comma {
                        break;
                    }
                    token_handler.next_token();
                }
            } else {
                token_handler.next_token();
            }

            if *token_handler.get_token() != Token::CSquare {
                return Err(token_handler.new_err(ET::ExpectedCSquare));
            }

            Ok(node)
        }

        Token::OParen => {
            token_handler.next_token();
            match arithmetic_expression(token_handler) {
                Ok(node) => {
                    if *token_handler.get_token() == Token::CParen {
                        Ok(node)
                    } else {
                        Err(token_handler.new_err(ET::ExpectedCParen))
                    }
                }
                Err(err) => Err(err),
            }
        }
        _ => Err(token_handler.new_err(ET::ExpectedExpression)),
    };
    token_handler.next_token();
    return ret;
}

fn assignment(token_handler: &mut TokenHandler, name: String) -> Result<TokenNode, RhErr> {
    dbg_println!("Assignment token: {:?}", token_handler.get_token());
    if *token_handler.peek(1) == Token::OSquare {
        token_handler.next_token();
        return deref_assignment(token_handler, Some(name.clone()));
    }

    token_handler.next_token();
    let assignment_tok = AssignmentOpType::from_token(token_handler.get_token()).unwrap();

    token_handler.next_token();
    let name_token = TokenNode::new(NodeType::Id(name.clone()), None);
    let token = TokenNode::new(
        NodeType::Assignment(assignment_tok),
        Some(vec![name_token, arithmetic_expression(token_handler)?]),
    );
    if *token_handler.get_token() != Token::Semi {
        return Err(token_handler.new_err(ET::ExpectedSemi));
    }

    Ok(token)
}

// Token coming in should be (, id or [
// if [] => Some(name)
// else => None
fn deref_assignment(
    token_handler: &mut TokenHandler,
    name: Option<String>,
) -> Result<TokenNode, RhErr> {
    let first = token_handler.get_token().clone();
    dbg_println!("DeRef Assignment First: {:?}", first);

    let token = match first {
        Token::OSquare => {
            token_handler.next_token();
            let post_mul = TokenNode::new(
                NodeType::Mul,
                vec![
                    arithmetic_expression(token_handler)?,
                    TokenNode::new(NodeType::NumLiteral(8), None),
                ]
                .into(),
            );
            let post_mul = TokenNode::new(
                NodeType::Sub,
                vec![
                    TokenNode::new(
                        NodeType::Id(
                            name.expect("Array assignments must have ids with names")
                                .clone(),
                        ),
                        None,
                    ),
                    post_mul,
                ]
                .into(),
            );
            if *token_handler.get_token() != Token::CSquare {
                return Err(token_handler.new_err(ET::ExpectedCSquare));
            }
            token_handler.next_token();

            post_mul
        }
        _ => arithmetic_expression(token_handler)?,
    };

    let deref_token = TokenNode::new(NodeType::DeRef, vec![token].into());
    let assignment_tok = AssignmentOpType::from_token(token_handler.get_token()).unwrap();
    token_handler.next_token();
    let token = TokenNode::new(
        NodeType::Assignment(assignment_tok),
        vec![deref_token, arithmetic_expression(token_handler)?].into(),
    );
    if *token_handler.get_token() != Token::Semi {
        return Err(token_handler.new_err(ET::ExpectedSemi));
    }

    Ok(token)
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
        .expect("While children to be some")
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

    dbg_println!("Post condition if token: {:?}", token_handler.get_token());
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

fn function_declare_statement(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    dbg_println!("Function Declaration");
    dbg_println!("Function Return Type: {:?}", token_handler.get_token());
    let t = if let Token::Type(t) = token_handler.get_token().clone() {
        t
    } else {
        panic!("Expected type");
    };
    token_handler.next_token();
    let token = token_handler.get_token();
    dbg_println!("Token: {:?}", token);
    if let Token::Id(id) = token {
        let mut function_node = TokenNode::new(
            NodeType::FunctionDeclaration((id.clone(), t.clone())),
            Some(vec![]),
        );
        token_handler.next_token();
        if *token_handler.get_token() != Token::OParen {
            return Err(token_handler.new_err(ET::ExpectedCParen));
        }
        token_handler.next_token();
        loop {
            let t = match token_handler.get_token() {
                Token::Type(t) => t,
                _ => break,
            };
            let declaration_node = declaration(token_handler, t.clone())?;
            function_node
                .children
                .as_mut()
                .unwrap()
                .push(declaration_node);
            dbg_println!("token: {:?}", token_handler.get_token());
            if *token_handler.get_token() != Token::Comma {
                break;
            }
            token_handler.next_token();
        }
        // token_handler.next_token();
        dbg_println!("Cparent: {:?}", token_handler.get_token());
        if *token_handler.get_token() != Token::CParen {
            return Err(token_handler.new_err(ET::ExpectedCParen));
        }
        token_handler.next_token();
        // TODO: Decide if we want to remove
        if *token_handler.get_token() == Token::Arrow {
            token_handler.next_token();
            if let Token::Type(t) = token_handler.get_token() {
                function_node
                    .children
                    .unwrap()
                    .push(TokenNode::new(NodeType::Type(t.clone()), None));
            }
            return Err(token_handler.new_err(ET::ExpectedType));
        }
        dbg_println!("Pre Scope Token: {:?}", token_handler.get_token());
        token_handler.next_token();
        let scope_node = scope(token_handler, ScopeType::Function(t.clone()))?;
        function_node.children.as_mut().unwrap().push(scope_node);

        return Ok(function_node);
    }

    Err(token_handler.new_err(ET::ExpectedId))
}

fn function_call_statement(
    token_handler: &mut TokenHandler,
    name: String,
) -> Result<TokenNode, RhErr> {
    dbg_println!(
        "Function call statement node: {:?}",
        token_handler.get_token()
    );
    let call_node = function_call(token_handler, name)?;
    token_handler.next_token();
    dbg_println!("post call statement {:?}", token_handler.get_token());
    if *token_handler.get_token() != Token::Semi {
        return Err(token_handler.new_err(ET::ExpectedSemi));
    }
    Ok(call_node)
}

fn function_call(token_handler: &mut TokenHandler, name: String) -> Result<TokenNode, RhErr> {
    dbg_println!("Fucntion call node: {:?}", token_handler.get_token());
    let mut function_call_node = TokenNode::new(NodeType::FunctionCall(name), Some(vec![]));
    token_handler.next_token();
    if *token_handler.get_token() != Token::OParen {
        return Err(token_handler.new_err(ET::ExpectedOParen));
    }
    token_handler.next_token();
    loop {
        dbg_println!("Call arg: {:?}", token_handler.get_token());
        if *token_handler.get_token() == Token::CParen {
            break;
        }
        let arg_node = arithmetic_expression(token_handler)?;
        function_call_node.children.as_mut().unwrap().push(arg_node);
        if *token_handler.get_token() != Token::Comma {
            break;
        }
        token_handler.next_token();
    }
    dbg_println!("post args token: {:?}", token_handler.get_token());
    if *token_handler.get_token() != Token::CParen {
        return Err(token_handler.new_err(ET::ExpectedCParen));
    }
    Ok(function_call_node)
}

fn id_statement(token_handler: &mut TokenHandler, id: String) -> Result<TokenNode, RhErr> {
    dbg_println!("id statement token: {:?}", token_handler.get_token());
    match token_handler.peek(1) {
        Token::OParen => function_call_statement(token_handler, id),
        _ => assignment(token_handler, id),
    }
}

fn type_statement(token_handler: &mut TokenHandler, t: RhTypes) -> Result<TokenNode, RhErr> {
    match token_handler.peek(2) {
        Token::OParen => function_declare_statement(token_handler),
        _ => declaration_statement(token_handler, t),
    }
}

fn condition(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    // let condition_node = TokenNode::new(NodeType::Condition());
    // token_handler.next_token();
    dbg_println!("\nOpening condition token: {:?}", token_handler.get_token());
    match token_handler.get_token() {
        Token::OParen => {
            // evaluate condition
            token_handler.next_token();
            let condition = condition_expr(token_handler);
            dbg_println!("Post condition token: {:?}", token_handler.get_token());
            //token_handler.next_token();
            match token_handler.get_token() {
                Token::CParen => condition,
                _ => {
                    dbg_println!("post condition {:?}\n", token_handler.get_token());
                    Err(token_handler.new_err(ET::ExpectedCParen))
                }
            }
        }
        _ => Err(token_handler.new_err(ET::ExpectedOParen)),
    }
}

fn condition_expr(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut left = condition_term(token_handler)?;
    dbg_println!("Condition Expr Left: {:?}", left);
    let mut curr = token_handler.get_token().clone();
    dbg_println!("cond expr curr: {:?}", curr);
    while curr == Token::AndCmp || curr == Token::OrCmp {
        token_handler.next_token();
        let right = if *token_handler.get_token() == Token::OParen {
            token_handler.next_token();
            let expr = condition_expr(token_handler)?;
            if *token_handler.get_token() != Token::CParen {
                return Err(token_handler.new_err(ET::ExpectedCParen));
            }

            token_handler.next_token();
            expr
        } else {
            condition_term(token_handler)?
        };
        left = TokenNode::new(
            NodeType::from_token(&curr).unwrap(),
            Some(vec![left, right]),
        );
        curr = token_handler.get_token().clone();
        dbg_println!("\nCondition expr curr: {:?}", curr);
    }
    Ok(left)
}

fn condition_term(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut left = arithmetic_expression(token_handler)?;
    dbg_println!("Left factor: {:?}", left);
    let mut curr = token_handler.get_token().clone();
    while curr == Token::NeqCmp || curr == Token::EqCmp {
        token_handler.next_token();
        let right = condition_factor(token_handler)?;
        println!("Right factor: {:?}", right);
        left = TokenNode::new(
            NodeType::from_token(&curr).unwrap(),
            Some(vec![left, right]),
        );
        curr = token_handler.get_token().clone();
        println!("curr: {:?}", curr);
    }
    Ok(left)
}

fn condition_factor(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    dbg_println!("Condition factor token: {:?}", token_handler.get_token());
    match token_handler.get_token() {
        Token::OParen => {
            token_handler.next_token();
            let expr = condition_expr(token_handler);
            dbg_println!("Post arith token: {:?}", token_handler.get_token());
            if *token_handler.get_token() != Token::CParen {
                return Err(token_handler.new_err(ET::ExpectedCParen));
            }
            expr
        }
        _ => arithmetic_expression(token_handler),
    }
}

fn asm_statement(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    token_handler.next_token();
    if *token_handler.get_token() != Token::OParen {
        return Err(token_handler.new_err(ET::ExpectedOParen));
    }
    token_handler.next_token();
    match token_handler.get_token().clone() {
        Token::StrLiteral(str) => {
            dbg_println!("Asm string: {}", str);
            token_handler.next_token();
            if *token_handler.get_token() != Token::CParen {
                return Err(token_handler.new_err(ET::ExpectedCParen));
            }
            token_handler.next_token();
            if *token_handler.get_token() != Token::Semi {
                dbg_println!("TOKEN: {:?}", token_handler.get_token());
                return Err(token_handler.new_err(ET::ExpectedSemi));
            }
            return Ok(TokenNode::new(NodeType::Asm(str.to_string()), None));
        }
        _ => return Err(token_handler.new_err(ET::ExpectedStrLiteral)),
    }
}

/// TODO: Make this function check for semi-colonons
fn for_statement(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    let mut for_node = TokenNode::new(NodeType::If, Some(vec![]));
    let t = match token_handler.get_token() {
        Token::Type(t) => t,
        _ => return Err(token_handler.new_err(ET::ExpectedType)),
    };
    let declare_node = declaration(token_handler, t.clone())?;
    for_node
        .children
        .as_mut()
        .expect("vec to be some")
        .push(declare_node);
    dbg_println!("token: {:?}, should be ;", token_handler.get_token());
    token_handler.next_token(); // might make semi handled by the called functions instead
    let condition_node = condition(token_handler)?;
    for_node
        .children
        .as_mut()
        .expect("vec to be some")
        .push(condition_node);
    token_handler.next_token();
    let statement_node = statement(token_handler, ScopeType::For)?;
    for_node
        .children
        .as_mut()
        .expect("vec to be some")
        .push(statement_node);
    token_handler.next_token();
    Ok(for_node)
}

pub fn assert_statement(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    token_handler.next_token();
    if *token_handler.get_token() != Token::OParen {
        return Err(token_handler.new_err(ET::ExpectedOParen));
    }
    token_handler.next_token();

    let condition_node = condition_expr(token_handler)?;

    let node = TokenNode::new(NodeType::Assert, vec![condition_node].into());

    if *token_handler.get_token() == Token::CParen {
        return Err(token_handler.new_err(ET::ExpectedCParen));
    }
    token_handler.next_token();
    if *token_handler.get_token() != Token::Semi {
        return Err(token_handler.new_err(ET::ExpectedSemi));
    }

    return Ok(node);
}

pub fn putchar_statement(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    token_handler.next_token();
    if *token_handler.get_token() != Token::OParen {
        return Err(token_handler.new_err(ET::ExpectedOParen));
    }
    token_handler.next_token();
    let expr_node = arithmetic_expression(token_handler)?;
    let putchar_node = TokenNode::new(NodeType::PutChar, Some(vec![expr_node]));
    dbg_println!("putchar token after: {:?}", token_handler.get_token());
    if *token_handler.get_token() != Token::CParen {
        return Err(token_handler.new_err(ET::ExpectedCParen));
    }
    token_handler.next_token();
    if *token_handler.get_token() != Token::Semi {
        return Err(token_handler.new_err(ET::ExpectedSemi));
    }
    return Ok(putchar_node);
}

pub fn return_statement(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    token_handler.next_token();
    if *token_handler.get_token() != Token::OParen {
        return Err(token_handler.new_err(ET::ExpectedOParen));
    }
    token_handler.next_token();
    let expr_node = condition_expr(token_handler)?;
    dbg_println!("post return {:?}", token_handler.get_token());
    if *token_handler.get_token() != Token::CParen {
        return Err(token_handler.new_err(ET::ExpectedCParen));
    }
    token_handler.next_token();
    if *token_handler.get_token() != Token::Semi {
        return Err(token_handler.new_err(ET::ExpectedSemi));
    }
    let return_token = TokenNode::new(NodeType::Return, Some(vec![expr_node]));
    return Ok(return_token);
}

pub fn struct_statement(token_handler: &mut TokenHandler) -> Result<TokenNode, RhErr> {
    token_handler.next_token();
    let id_token = token_handler.get_token().clone();
    token_handler.next_token();
    let curl_token = token_handler.get_token();
    match (id_token, curl_token) {
        (Token::Id(_id), Token::OCurl) => return Err(token_handler.new_err(ET::ExpectedId)),
        _ => {}
    }

    let mut parameters: Vec<TokenNode> = vec![];
    while let Token::Type(t) = token_handler.get_token() {
        parameters.push(declaration_statement(token_handler, t.clone())?);
    }
    let declaration_token = TokenNode::new(NodeType::Return, Some(parameters));
    if *token_handler.get_token() != Token::CCurl {
        return Err(token_handler.new_err(ET::ExpectedCCurl));
    }
    return Ok(declaration_token);
}
