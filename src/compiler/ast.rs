use crate::compiler::lexer::{RhType, Token};
#[derive(Debug, PartialEq, Clone)]
pub enum ScopeType {
    Function(RhType),
    While,
    Program,
    If,
    Loop,
    _For,
}

// Valid Node Types
#[derive(Debug, Clone, PartialEq)]
pub enum TokenNode {
    // Scope
    Program,
    Scope(Option<RhType>),

    // Expression Nodes
    Id(String),
    Int32Lit(usize),

    DeRef(Box<TokenNode>),
    Adr(String),
    IndexArray {
        id: String,
        expr: Box<TokenNode>,
    },

    // Statements
    Declaration {
        lside: String,
        t: RhType,
        rside: Box<TokenNode>,
    },
    Assignment {
        lside: String,
        op: AssignmentOpType,
        rside: Box<TokenNode>,
    },

    FunctionCall {
        name: String,
        args: Vec<TokenNode>,
    },
    DerefAssignment {
        lside: Box<TokenNode>,
        op: AssignmentOpType,
        rside: Box<TokenNode>,
    },

    If {
        condition: Box<TokenNode>,
        scope: Vec<TokenNode>,
    },
    For {
        init: Box<TokenNode>,
        condition: Box<TokenNode>,
        incr: Box<TokenNode>,
        scope: Vec<TokenNode>,
    },
    While {
        condition: Box<TokenNode>,
        scope: Vec<TokenNode>,
    },
    Break,

    PtrDeclaration {
        lside: String,
        t: RhType,
        rside: Box<TokenNode>,
    },
    ArrayDeclaration {
        lside: String,
        t: RhType,
        size: usize,
        rside: Vec<TokenNode>,
    },

    IndexArrayAssignment {
        id: String,
        rside: Box<TokenNode>,
        lside: Box<TokenNode>,
    },
    FunctionDeclaration {
        name: String,
        ret: RhType,
    },
    Return {
        expr: Box<TokenNode>,
    },
    StructDefinition {
        struct_id: String,
        field_definitions: Vec<(String, usize, RhType)>, // id, ptr_count, underlying type
    },
    StructDeclaration {
        var_id: String,
        struct_id: String,
        exprs: Vec<TokenNode>,
    }, // expr nodes
    StructFieldAssignment {
        // TODO Check if we still need
        var_id: String,
        field_id: String,
        assignment_op: AssignmentOpType,
        expr: Box<TokenNode>,
    },
    StructFieldId {
        var_id: String,
        field_id: String,
    },

    PutChar(Box<TokenNode>),
    Assert(Box<TokenNode>),
    Asm(String),

    // Operators
    Sub,
    Div,
    Eq,
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
    Add,
    DivEq,
    MulEq,
    Mul,
    AndCmp,
    OrCmp,
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
    AddO,
}

impl AssignmentOpType {
    pub fn from_token(tok: &Token) -> Result<AssignmentOpType, ()> {
        match tok {
            Token::Eq => Ok(AssignmentOpType::Eq),
            Token::SubEq => Ok(AssignmentOpType::SubEq),
            Token::AddEq => Ok(AssignmentOpType::AddEq),
            Token::AddO => Ok(AssignmentOpType::AddO),
            Token::DivEq => Ok(AssignmentOpType::DivEq),
            Token::MulEq => Ok(AssignmentOpType::MulEq),
            Token::BOrEq => Ok(AssignmentOpType::BOrEq),
            Token::BAndEq => Ok(AssignmentOpType::BAndEq),
            Token::BXorEq => Ok(AssignmentOpType::BXorEq),
            _ => {
                println!("Oh God No, Not A Valid OpEq Token: {:?}", tok);
                return Err(());
            }
        }
    }
}

impl std::fmt::Display for AssignmentOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            AssignmentOpType::Eq => "=",
            AssignmentOpType::SubEq => "-=",
            AssignmentOpType::DivEq => "/=",
            AssignmentOpType::AddEq => "+=",
            AssignmentOpType::MulEq => "*=",
            AssignmentOpType::AddO => "++",
            AssignmentOpType::BOrEq => "|=",
            AssignmentOpType::BXorEq => "^=",
            AssignmentOpType::BAndEq => "&=",
        };

        write!(f, "{}", op)
    }
}

impl NodeType {
    pub fn from_token(tok: &Token) -> Result<NodeType, ()> {
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

#[derive(Debug, Clone, PartialEq)]
pub struct TokenNode {
    pub token: NodeType,
    pub line: usize,
    pub children: Option<Box<[TokenNode]>>,
}

impl std::fmt::Display for TokenNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.token) // doesn't print values
    }
}

impl TokenNode {
    pub fn new(token: NodeType, children: Option<Box<[TokenNode]>>, line: usize) -> TokenNode {
        TokenNode {
            token,
            line,
            children,
        }
    }
    pub fn print(&self, n: &mut i32) {
        (0..*n).into_iter().for_each(|_| print!("  "));
        println!("{:?}", self);
        *n += 1;
        if let Some(children) = self.children.as_ref() {
            children.iter().for_each(|node| {
                node.print(n);
            });
        }
        *n -= 1;
        // println!("End Children");
    }
}
