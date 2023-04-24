use std::error::Error;

use aiken_lang::{parser::error::ParseError, tipo};
use miette::Diagnostic;

#[derive(Clone)]
pub enum CompilerError {
    Parse(ParseError),
    Type(tipo::error::Error),
}

impl CompilerError {
    pub fn message(&self) -> String {
        match self {
            CompilerError::Parse(p) => p
                .source()
                .map_or_else(|| p.to_string(), |ps| ps.to_string()),
            CompilerError::Type(t) => t
                .source()
                .map_or_else(|| t.to_string(), |ts| ts.to_string()),
        }
    }

    pub fn code(&self) -> Option<String> {
        match self {
            CompilerError::Parse(p) => p.code().map(|pc| pc.to_string()),
            CompilerError::Type(t) => t.code().map(|tc| tc.to_string()),
        }
    }

    pub fn help(&self) -> Option<String> {
        match self {
            CompilerError::Parse(p) => p.help().map(|ph| ph.to_string()),
            CompilerError::Type(t) => t.help().map(|th| th.to_string()),
        }
    }
}
