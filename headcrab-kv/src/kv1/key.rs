use std::fmt::Display;

/// A generic key with a string value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key(pub String, pub String);

impl Key {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self(key.into(), value.into())
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{}" "{}""#, self.0, self.1)
    }
}
