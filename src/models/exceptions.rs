use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DictionaryOverride {
    pub word: String,
    pub phonemes: Vec<String>,
    pub is_case_sensitive: bool,
    pub note_comment: Option<String>,
}

impl DictionaryOverride {
    pub fn new(word: impl Into<String>, phonemes: Vec<String>) -> Self {
        Self {
            word: word.into(),
            phonemes,
            is_case_sensitive: false,
            note_comment: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegexRule {
    pub name: String,
    pub pattern: String,
    pub replacement: String,
    pub enabled: bool,
    pub order: i32,
}

impl RegexRule {
    pub fn new(name: impl Into<String>, pattern: impl Into<String>, replacement: impl Into<String>, order: i32) -> Self {
        Self {
            name: name.into(),
            pattern: pattern.into(),
            replacement: replacement.into(),
            enabled: true,
            order,
        }
    }
}
