use snafu::prelude::*;

/// Each variant wraps the line numberi the error was found on
#[derive(Debug, Clone, strum::Display)]
pub enum ET {
    ExpectedCParen,
    ExpectedExpression,
    ExpectedId,
    UndeclaredId,
    ExpectedAssignment,
    ExpectedStatement,
    ExpectedCondition,
    ExpectedOParen,
    ExpectedCCurl,
    ExpectedStrLiteral,
}

pub struct RhErr {
    err: ET,
    line: i32,
}
impl Error for RhErr {}
