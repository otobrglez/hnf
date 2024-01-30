use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum HNError {
    Http(String),
    Json(String),
    Casting(String),
}

impl From<ureq::Error> for HNError {
    fn from(error: ureq::Error) -> Self {
        match error {
            ureq::Error::Transport(e) => HNError::Http(e.to_string()),
            other => HNError::Http(other.to_string()),
        }
    }
}

impl From<serde_json::Error> for HNError {
    fn from(error: serde_json::Error) -> Self {
        HNError::Casting(error.to_string())
    }
}

impl Display for HNError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let other = self;
        f.write_str(&format!("HNError with {}", other))
    }
}
