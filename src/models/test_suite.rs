use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestCase {
    pub id: String,
    pub name: String,
    pub lyrics_input: String,
    pub tone_input: String,
    pub bpm: f64,
    pub expected_phonemes: Vec<String>,
    pub expected_durations_ms: Option<Vec<i32>>,
}

impl Default for TestCase {
    fn default() -> Self {
        Self {
            id: uuid_short(),
            name: "Novo Teste".to_string(),
            lyrics_input: "la la".to_string(),
            tone_input: "C4".to_string(),
            bpm: 120.0,
            expected_phonemes: vec!["- la".to_string(), "la".to_string()],
            expected_durations_ms: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestExecutionResult {
    pub test_case_id: String,
    pub passed: bool,
    pub actual_phonemes: Vec<String>,
    pub diff_message: String,
    pub duration_ms: u128,
}

fn uuid_short() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}", (nanos & 0xFFFFFFFF))
}
