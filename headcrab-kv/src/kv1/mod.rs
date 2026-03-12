use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use logos::Logos;
use thiserror::Error;

mod block;
mod key;
mod tokens;

#[cfg(test)]
mod tests;

pub use block::*;
pub use key::*;

use tokens::*;

pub type Result<T> = std::result::Result<T, KV1Error>;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KV1Tree {
    pub blocks: Vec<Block>,
}

impl KV1Tree {
    pub fn parse_from_file(path: impl AsRef<Path>) -> Result<Self> {
        path.as_ref().try_into()
    }
}

impl TryFrom<&Path> for KV1Tree {
    type Error = KV1Error;

    fn try_from(value: &Path) -> Result<Self> {
        fs::read_to_string(value)
            .map_err(|e| KV1Error::new(value.to_owned(), e.into()))?
            .parse::<KV1Tree>()
            .map_err(|e| KV1Error::new(value.to_owned(), e.into()))
    }
}

impl FromStr for KV1Tree {
    type Err = ParsingError;

    fn from_str(s: &str) -> std::result::Result<Self, ParsingError> {
        let mut blocks: Vec<Block> = vec![];
        let mut lexer = KV1Token::lexer(s);
        let mut current_block: Vec<Block> = vec![];

        while let Some(token) = lexer.next().transpose()? {
            match token {
                KV1Token::Block => current_block.push(Block::new(lexer.slice())),
                KV1Token::Pair(pair) => current_block
                    .last_mut()
                    .ok_or(ParsingError::MissingBlock)?
                    .add_pair(pair),
                KV1Token::LeftBrace => {}
                KV1Token::RightBrace => {
                    let current = current_block.pop().ok_or(ParsingError::MissingBlock)?;
                    if let Some(cb) = current_block.last_mut() {
                        cb.add_block(current)
                    } else {
                        blocks.push(current);
                    }
                }
            }
        }

        Ok(KV1Tree { blocks: blocks })
    }
}

impl Display for KV1Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.blocks {
            write!(f, "{b}")?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
#[error("{p}: {kind}")]
pub struct KV1Error {
    p: PathBuf,
    kind: ErrorKind,
}

impl KV1Error {
    fn new(p: PathBuf, kind: ErrorKind) -> Self {
        Self { p, kind }
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parsing error: {0}")]
    Parsing(#[from] ParsingError),
}

#[allow(dead_code)]
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ParsingError {
    #[error("lexing error: {0}")]
    Lexing(#[from] LexingError),
    #[error("expected block not found")]
    MissingBlock,
}
