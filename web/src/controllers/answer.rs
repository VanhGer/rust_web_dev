use warp::http::StatusCode;
use handle_errors::CustomError;
use std::collections::HashMap;
use crate::store::Store;
use crate::types::account::Session;
use crate::types::answer::NewAnswer;
use crate::types::pagination::Pagination;
use tracing::{instrument, info};
use tracing::{event, Level};

use crate::types::pagination::extract_pagination;

/// Add an answer to a question from `/answers` route
/// # Example query
/// POST requests to this route, with the body format is
/// x-www-form-urlendcoded with two key-value:
/// content: hellomn
/// question_id: 2
#[instrument]
pub async fn add_answer(
    session: Session,
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {

    let account_id = session.account_id;
    let answer = NewAnswer {
        content: new_answer.content,
        question_id: new_answer.question_id,
    };

    match store.add_answer(answer, account_id).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Answer added", StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
/// This function gets answers to a specific question from '/answer' route
/// # Example query
/// GET requests to this route, with the query params:
/// limit, offset and question_id
/// ```
/// /answers?limit=10&offset=0&question_id=1
/// ```
#[instrument]
pub async fn get_question_answers(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "web", Level::INFO, "querying question's answers");
    let mut pagination = Pagination::default(); 
    let mut question_id = 0;
    
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        question_id = params.get("question_id").unwrap().parse::<i32>()
                    .map_err(CustomError::ParseError)?;
        pagination = extract_pagination(params)?;
    }
    
    match store.get_question_answers(pagination.limit, pagination.offset, question_id).await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

