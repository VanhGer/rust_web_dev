use handle_errors::CustomError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::http::StatusCode;
use tracing::{instrument, info};

use crate::types::pagination::Pagination;
use tracing::{event, Level};
use crate::types::question::NewQuestion;
use crate::profanity::check_profanity;

use crate::{
    store::Store,
    types::{
        pagination::extract_pagination,
        question::{Question, QuestionId},
    },
};



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



pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    
    let title = match check_profanity(new_question.title).await { 
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let content = match check_profanity(new_question.content).await { 
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    
    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    match store.add_question(question).await {
        Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }

}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {

    let title = tokio::spawn(check_profanity(question.title));

    let content = tokio::spawn(check_profanity(question.content)); 

    let (title, content) = (title.await.unwrap(), content.await.unwrap());
    if title.is_err() {
        return Err(warp::reject::custom(title.unwrap_err()));
    } 

    if content.is_err() { 
        return Err(warp::reject::custom(content.unwrap_err()));
    }


    let question = Question {
        id: question.id,
        title: title.unwrap(),
        content: content.unwrap(),
        tags: question.tags,
    };
    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
