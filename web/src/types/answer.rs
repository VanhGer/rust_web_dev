use serde::{Deserialize, Serialize};

use crate::types::question::QuestionId;
/// Represents the unique identifier for an answer.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub i32);
/// Represents an answer to a question
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
     /// Unique identifier of the answer.
    pub id: AnswerId,
    /// Content of the answer.
    pub content: String,
    /// ID of the question to which the answer belongs.
    pub question_id: QuestionId,
}

/// Represents a new answer to be added, without its identifier.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewAnswer {
    /// Content of the new answer.
    pub content: String,
    /// ID of the question to which the new answer belongs.
    pub question_id: QuestionId,
}
