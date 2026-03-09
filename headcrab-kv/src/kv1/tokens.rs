use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip "[[:space:]]+")]
#[logos(subpattern quoted = r#"\"(?:[^\\\"]|\\.)+\""#)]
#[logos(subpattern non_quoted = r#"[^\"\s{}]+"#)]
pub enum KV1Token {
    #[regex("[[:alnum:]]+")]
    Block,
    #[regex("(?:(?&quoted)|(?&non_quoted))[ \t]+(?:(?&quoted)|(?&non_quoted))")]
    Pair,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
}
