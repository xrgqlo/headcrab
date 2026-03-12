use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use logos::Logos;
use thiserror::Error;

mod block;
mod key;
mod tokens;

pub use block::*;
pub use key::*;

use tokens::*;

pub type Result<T> = std::result::Result<T, ParsingError>;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub struct KV1Tree {
    pub blocks: Vec<Block>,
}

impl KV1Tree {
    pub fn parse_from_file(path: impl AsRef<Path>) -> Result<Self> {
        path.as_ref().try_into()
    }
}

impl TryFrom<&Path> for KV1Tree {
    type Error = ParsingError;

    fn try_from(value: &Path) -> Result<Self> {
        let content =
            fs::read_to_string(value).map_err(|e| ParsingError::new(value.to_owned(), e.into()))?;
        KV1Tree::from_str(&content).map_err(|e| ParsingError::new(value.to_owned(), e.into()))
    }
}

impl FromStr for KV1Tree {
    type Err = LexerError;

    fn from_str(s: &str) -> std::result::Result<Self, LexerError> {
        let mut blocks: Vec<Block> = vec![];
        let mut lexer = KV1Token::lexer(s);
        let mut current_block: Vec<Block> = vec![];
        let mut depth = 0;

        while let Some(token) = lexer
            .next()
            .transpose()
            .map_err(|_| LexerError::Placeholder)?
        {
            // needs proper error handling but whatever

            match token {
                KV1Token::Block => current_block.push(Block::new(lexer.slice())),
                KV1Token::Pair => {
                    let mut split = lexer.slice().split("\"");
                    split.next();
                    let name = split.next().unwrap();
                    split.next();
                    let value = split.next().unwrap();
                    current_block[depth - 1].add_pair(name, value);
                }
                KV1Token::LeftBrace => depth += 1,
                KV1Token::RightBrace => {
                    depth -= 1;
                    if depth > 0 {
                        let current = current_block[depth].clone();
                        current_block[depth - 1].blocks.push(current);
                    } else {
                        blocks.push(current_block[depth].clone());
                        current_block = vec![]
                    }
                }
            }
        }

        Ok(KV1Tree { blocks: blocks })
    }
}

impl ToString for KV1Tree {
    fn to_string(&self) -> String {
        let mut string = "".to_string();

        for block in &self.blocks {
            for line in block.to_strings() {
                string += line.as_str();
            }
        }

        string
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
#[error("could not parse {p}: {kind}")]
pub struct ParsingError {
    p: PathBuf,
    kind: ParsingErrorKind,
}

impl ParsingError {
    fn new(p: PathBuf, kind: ParsingErrorKind) -> Self {
        Self { p, kind }
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ParsingErrorKind {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Lexer(#[from] LexerError),
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum LexerError {
    #[error("placeholder lexing error")]
    Placeholder,
}
