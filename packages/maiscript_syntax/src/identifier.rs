use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifierName(String);

impl Display for IdentifierName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for IdentifierName {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for IdentifierName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
