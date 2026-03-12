use std::{fmt::Display, str::FromStr};

use thiserror::Error;

/// A generic key with a string value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key(pub String, pub String);

impl Key {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self(key.into(), value.into())
    }
}

/// Trim quotes around a string once if they exist
fn trim_quotes(s: &str) -> Result<&str, PairParsingErrorKind> {
    if s.starts_with("\"") {
        s.strip_prefix("\"")
            .map(|s| s.strip_suffix("\""))
            .flatten()
            .ok_or(PairParsingErrorKind::MissingQuotes)
    } else {
        Ok(s)
    }
}

impl FromStr for Key {
    type Err = PairParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = s.split_whitespace();
        let name = p
            .next()
            .ok_or_else(|| PairParsingErrorKind::MissingName)
            .and_then(|n| trim_quotes(n))
            .map_err(|kind| PairParsingError {
                s: s.to_owned(),
                kind,
            })?
            .to_owned();

        let value = p
            .next()
            .ok_or_else(|| PairParsingErrorKind::MissingValue)
            .and_then(|v| trim_quotes(v))
            .map_err(|kind| PairParsingError {
                s: s.to_owned(),
                kind,
            })?
            .to_owned();

        Ok(Key::new(name, value))
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{}" "{}""#, self.0, self.1)
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
#[error("error parsing pair {s}: {kind}")]
pub struct PairParsingError {
    s: String,
    kind: PairParsingErrorKind,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub(super) enum PairParsingErrorKind {
    #[error("the pair is missing a name")]
    MissingName,
    #[error("the pair is missing a value")]
    MissingValue,
    #[error("the token is missing a quote")]
    MissingQuotes,
}
