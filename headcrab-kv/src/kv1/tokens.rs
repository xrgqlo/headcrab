use logos::Logos;
use thiserror::Error;

use super::Key;

#[derive(Debug, Logos, PartialEq)]
#[logos(error=LexingError)]
#[logos(skip "[[:space:]]+")]
#[logos(subpattern quoted = r#"\"(?:[^\\\"]|\\.)+\""#)]
#[logos(subpattern non_quoted = r#"[^\"\s{}]+"#)]
pub(super) enum KV1Token {
    #[regex("[[:alnum:]]+")]
    Block,
    #[regex("(?:(?&quoted)|(?&non_quoted))[ \t]+(?:(?&quoted)|(?&non_quoted))", |lex| lex.slice().parse())]
    Pair(Key),
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
}

#[derive(Debug, Error, Default, Clone, PartialEq)]
pub enum LexingError {
    #[default]
    #[error("placeholder lexing error")]
    Placeholder,
    #[error("could not parse pair: {0}")]
    InvalidPair(#[from] super::PairParsingError),
}
