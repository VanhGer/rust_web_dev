
use std::collections::HashMap;
use warp::http::StatusCode;
use tracing::{instrument};

use crate::types::pagination::Pagination;
use tracing::{event, Level};
use crate::types::question::NewQuestion;
use crate::types::account::Session;

use crate::{
    store::Store,
    types::{
        pagination::extract_pagination,
        question::{Question},
    },
};

/// This function gets a list of all the questions from '/questions' route
/// # Example query
/// GET requests to this route, with the query params:
/// limit, offset
/// ```
/// /answers?limit=10&offset=0&question_id=1
/// ```

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "web", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }
    match store.get_questions(pagination.limit, pagination.offset).await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

/// Add a question from `/questions` route
/// # Example query
/// POST requests to this route, with the body format is
/// json with 3 key-value:
///```
/// {
///     "title": "Tai vi sao",
///     "content": "Yeahh, cam xuc kia quay ve"
///     "tags": ["messi", "1a"]
/// }
///```
#[instrument]
pub async fn add_question(
    session: Session,
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    let question = NewQuestion {
        title: new_question.title,
        content: new_question.content,
        tags: new_question.tags,
    };

    match store.add_question(question, account_id).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

/// Update an existing question from `/questions` route
/// # Example query
/// PUT requests to this route, with the body format is
/// json with the id of the question and any of 3 key-value
/// we want to update:
///```
/// {
///     "id":   "3",  
///     "title": "Tai vi sao"
/// }
///```
#[instrument]
pub async fn update_question(
    id: i32,
    session: Session,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        let question = Question {
            id: question.id,
            title: question.title,
            content: question.content,
            tags: question.tags,
        };
        match store.update_question(question, id, account_id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }
        
    } else {
        Err(warp::reject::custom(handle_errors::CustomError::Unauthorized))
    }
}


/// Delete an existing question from `/questions/question_id` route
/// # Example query
/// DELETE requests to this route, with the query is
/// the id of the question we want to delete
///```
/// /questions/2
///```
#[instrument]
pub async fn delete_question(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        match store.delete_question(id, account_id).await {
            Ok(_) => Ok(warp::reply::with_status(
                format!("Question {} deleted", id),
                StatusCode::OK,
            )),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(handle_errors::CustomError::Unauthorized))
    }
}
