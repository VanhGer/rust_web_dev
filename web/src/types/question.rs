use serde::{Deserialize, Serialize};
/// This object epresents a question, including its unique identifier,
/// title, content and tags.
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

/// Represents the unique identifier for a question.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub i32);

/// Represents a new question to be added, without an identifier.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}